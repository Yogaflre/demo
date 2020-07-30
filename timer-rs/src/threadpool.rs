use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

enum Message {
    Task(Arc<dyn 'static + Send + Sync + Fn()>),
    Stop,
}

pub struct ThreadPool {
    sender: Sender<Message>,
    threads: Vec<Option<thread::JoinHandle<()>>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.threads
            .iter()
            .for_each(|_| self.sender.send(Message::Stop).expect("send stop error."));

        for thread in &mut self.threads {
            if let Some(t) = thread.take() {
                t.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: u8) -> ThreadPool {
        let mut threads = vec![];

        let (sender, receiver) = channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..size {
            let recv = receiver.clone();
            let thread = thread::spawn(move || loop {
                let message = recv.lock().unwrap().recv().expect("recv task error.");
                match message {
                    Message::Task(task) => task(),
                    Message::Stop => break,
                }
            });
            threads.push(Some(thread));
        }
        ThreadPool {
            sender: sender,
            threads: threads,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: 'static + Send + Fn() + Sync,
    {
        self.sender
            .send(Message::Task(Arc::new(f)))
            .expect("send task error.");
    }

    pub fn execute_arc(&self, f: Arc<dyn 'static + Send + Sync + Fn()>) {
        self.sender
            .send(Message::Task(f))
            .expect("send task error.");
    }
}
