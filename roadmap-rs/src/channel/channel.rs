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
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        let mut inner = self.channel.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                None if inner.senders == 0 => return None, // alse use Arc::strong_count(self.channel) == 1 to prove that no senders
                None => {
                    inner = self.channel.cond.wait(inner).unwrap();
                }
                t => return t,
            };
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
    senders: usize, // drop channel when senders is 0
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
        Receiver { channel },
    );
}

#[cfg(test)]
mod tests {
    use super::channel;

    #[test]
    fn transfer() {
        let (mut tx, mut rx) = channel();
        tx.send(String::from("hello"));
        assert_eq!(rx.recv().unwrap(), "hello");
    }

    #[test]
    fn last() {
        let (_, mut rx) = channel::<()>();
        assert_eq!(rx.recv(), None);
    }
}
