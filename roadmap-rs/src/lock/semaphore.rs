use std::sync::{Condvar, Mutex};

/*
 * Semaphore by mutex and cond
 */
pub struct Semaphore {
    sem_lock: Mutex<i32>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(sem: i32) -> Self {
        Self {
            sem_lock: Mutex::new(sem),
            cond: Condvar::new(),
        }
    }

    pub fn p(&self) {
        if let Ok(mut sem) = self.sem_lock.lock() {
            while *sem <= 0 {
                sem = self.cond.wait(sem).unwrap();
            }
            *sem -= 1;
        }
    }

    pub fn v(&self) {
        if let Ok(mut sem) = self.sem_lock.lock() {
            *sem += 1;
            self.cond.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lock::semaphore::Semaphore;
    use std::{sync::Arc, thread};

    #[test]
    fn thread_join() {
        let sem = Arc::new(Semaphore::new(0));
        let cloned_sem = sem.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
            println!("sub thread down");
            cloned_sem.v();
        });
        sem.p();
        println!("main thread down");
    }

    #[test]
    fn producer_consumer() {
        let num = Arc::new(0);
        let full = Semaphore::new(0);
        let empty = Semaphore::new(10);
        let lock = Semaphore::new(1);
    }
}
