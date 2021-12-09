use std::{future::Future, sync::Arc, thread};

use async_task::{Runnable, Task};
use crossbeam::channel::{Receiver, Sender};

struct Executor {
    queue: Arc<Sender<Runnable>>,
}

impl Executor {
    fn new() -> Self {
        let (tx, rx): (Sender<Runnable>, Receiver<Runnable>) = crossbeam::channel::unbounded();
        thread::spawn(move || {
            for runnable in rx.iter() {
                runnable.run();
            }
        });
        Self {
            queue: Arc::new(tx),
        }
    }

    fn spawn<F>(&self, f: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let queue_c = self.queue.clone();
        let schedule = move |runnable| queue_c.send(runnable).unwrap();
        let (runnable, task) = async_task::spawn(f, schedule);
        runnable.schedule();
        return task;
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::Executor;

    #[test]
    fn execute() {
        let f = async {
            thread::sleep(Duration::from_secs(1));
            println!("done!");
            return 0;
        };
        let executor = Executor::new();
        let task = executor.spawn(f);
        let res = futures_lite::future::block_on(task);
        assert_eq!(res, 0);
    }
}
