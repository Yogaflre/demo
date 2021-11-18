use std::{
    collections::LinkedList,
    sync::{Arc, Condvar, Mutex},
};

struct Sender<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.channel.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Self {
            channel: self.channel.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.channel.inner.lock().unwrap();
        inner.senders -= 1;
        let is_last = inner.senders == 0;
        drop(inner);
        if is_last {
            self.channel.cond.notify_one(); // FIXME Is is necessary?
        }
    }
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let mut inner = self.channel.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner); // NOTE Drop lock before notify a receiver make ensure that receiver can get the lock in first time
        self.channel.cond.notify_one();
    }
}

struct Receiver<T> {
    channel: Arc<Channel<T>>,
    buffer: LinkedList<T>, // Changing space for time
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        } else {
            let mut inner = self.channel.inner.lock().unwrap();
            loop {
                match inner.queue.pop_front() {
                    None if inner.senders == 0 => return None, // alse use Arc::strong_count(self.channel) == 1 to prove that no senders
                    None => {
                        inner = self.channel.cond.wait(inner).unwrap();
                    }
                    t => {
                        std::mem::swap(&mut inner.queue, &mut self.buffer); // TRICK receive all message at once! To avoid get lock every time
                        return t;
                    }
                };
            }
        }
    }

    fn try_recv(&mut self) -> Option<T> {
        return self.channel.inner.lock().unwrap().queue.pop_front();
    }
}

struct Channel<T> {
    inner: Mutex<Inner<T>>,
    cond: Condvar,
}

struct Inner<T> {
    queue: LinkedList<T>,
    senders: usize,
}

fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Mutex::new(Inner {
        queue: LinkedList::new(),
        senders: 1,
    });
    let channel = Arc::new(Channel {
        inner,
        cond: Condvar::new(),
    });

    return (
        Sender {
            channel: channel.clone(),
        },
        Receiver {
            channel,
            buffer: LinkedList::new(),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::channel;
    use std::thread;

    #[test]
    fn transfer() {
        let (mut tx, mut rx) = channel();
        tx.send(String::from("hello"));
        assert_eq!(rx.recv().unwrap(), "hello");
    }

    #[test]
    fn closed() {
        let (_, mut rx) = channel::<()>();
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn multipal() {
        let (mut tx, mut rx) = channel::<i32>();
        let sub = thread::spawn(move || {
            tx.send(1);
            tx.send(2);
        });
        sub.join().unwrap();
        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 2);
        assert_eq!(rx.recv(), None);
    }
}
