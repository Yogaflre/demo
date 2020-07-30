mod threadpool;
mod timer;

#[cfg(test)]
mod tests {
    use crate::timer::Timer;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    #[test]
    fn delay() {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start: {}", start);
        let mut timer = Timer::new(4);
        timer.delay(
            Duration::from_secs(1),
            Arc::new(move || {
                println!(
                    "delay: {} ms",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        - start
                );
            }),
        );
        timer.delay(
            Duration::from_secs(3),
            Arc::new(move || {
                println!(
                    "delay2: {} ms",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        - start
                );
            }),
        );
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn repeating() {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start: {}", start);
        let mut timer = Timer::new(4);
        timer.repeating(
            Duration::from_secs(3),
            Arc::new(move || {
                println!(
                    "repeat: {} ms",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        - start
                );
            }),
        );
        timer.repeating(
            Duration::from_secs(1),
            Arc::new(move || {
                println!(
                    "repeat2: {} ms",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        - start
                );
            }),
        );
        thread::sleep(Duration::from_secs(100));
    }

    use crate::threadpool::ThreadPool;
    #[test]
    fn thread_pool() {
        let pool = ThreadPool::new(2);
        pool.execute(|| {
            println!("1 task start");
            thread::sleep(Duration::from_secs(3));
            println!("1 task end");
        });
        pool.execute(|| {
            println!("2 task start");
            thread::sleep(Duration::from_secs(6));
            println!("2 task end");
        });
        pool.execute(|| {
            println!("3 task start");
            thread::sleep(Duration::from_secs(3));
            println!("3 task end");
        });
    }
}
