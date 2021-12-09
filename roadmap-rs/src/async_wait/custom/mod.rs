mod executor;
mod future;
mod reactor;
mod waker;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::{executor::Executor, future::Task, reactor::Reactor};

    #[test]
    fn timer() {
        let reactor = Reactor::new();

        let start = Instant::now();

        let future1 = async {
            Task::new(1, 500, reactor.clone()).await;
            println!("future1 elaspsed: {}ms", start.elapsed().as_millis());
        };
        let future2 = async {
            Task::new(2, 500, reactor.clone()).await;
            println!("future2 elaspsed: {}ms", start.elapsed().as_millis());
        };

        let future = async {
            future1.await;
            future2.await;
        };

        Executor::block_on(future);
    }
}

