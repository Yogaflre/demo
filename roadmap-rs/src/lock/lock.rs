use std::{
    collections::LinkedList,
    sync::atomic::{AtomicBool, Ordering},
    thread::{self, Thread},
};

use super::semaphore::Semaphore;

pub trait Lock
where
    Self: Send + Sync,
{
    fn new() -> Self
    where
        Self: Sized;
    fn lock(&self);
    fn unlock(&self);
}

/*
 * Lock by CAS & loop
 */
struct SpinLock {
    flag: AtomicBool,
}

impl Lock for SpinLock {
    fn new() -> Self {
        Self {
            flag: AtomicBool::new(true),
        }
    }

    fn lock(&self) {
        while let Err(_) =
            self.flag
                .compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed)
        {}
    }

    fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}

/*
 * Lock by CAS & park/unpark & Queue
 */
struct ParkLock {
    flag: AtomicBool,
    locked: bool,
    threads: LinkedList<Thread>,
}

impl Lock for ParkLock {
    fn new() -> Self {
        Self {
            flag: AtomicBool::new(true),
            locked: false,
            threads: LinkedList::new(),
        }
    }

    fn lock(&self) {
        while let Err(_) =
            self.flag
                .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        {}
        if self.locked {
            let threads = (&self.threads) as *const LinkedList<Thread> as *mut LinkedList<Thread>;
            unsafe {
                (&mut *threads).push_front(thread::current());
            }
            self.flag
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .unwrap();
            thread::park(); // park after release flag. If this thread is wakeup, it has already get locked.
        } else {
            let locked = (&self.locked) as *const bool as *mut bool;
            unsafe {
                *locked = true;
            }
            self.flag
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .unwrap();
        }
    }

    fn unlock(&self) {
        while let Err(_) =
            self.flag
                .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        {}
        let locked = (&self.locked) as *const bool as *mut bool;
        let threads = (&self.threads) as *const LinkedList<Thread> as *mut LinkedList<Thread>;
        unsafe {
            self.flag
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .unwrap();
            if let Some(t) = (&mut *threads).pop_back() {
                t.unpark(); // Do not set "locked = false", because locked pass to thread t.
            } else {
                *locked = false;
            }
        }
    }
}

/*
 * Lock by Semaphore
 */
struct SemLock {
    sem: Semaphore,
}

impl Lock for SemLock {
    fn new() -> Self {
        Self {
            sem: Semaphore::new(1),
        }
    }

    fn lock(&self) {
        self.sem.p();
    }

    fn unlock(&self) {
        self.sem.v();
    }
}

/*
 * RwLock by Semaphore
 */
struct SemRwLock {
    flag: Semaphore,
    lock: Semaphore,
    reader: usize,
}

impl SemRwLock {
    fn new() -> Self {
        Self {
            flag: Semaphore::new(1),
            lock: Semaphore::new(1),
            reader: 0,
        }
    }

    fn read_lock(&self) {
        self.flag.p();
        unsafe {
            *((&self.reader) as *const usize as *mut usize) += 1;
        }
        if self.reader == 1 {
            self.lock.p();
        }
        self.flag.v();
    }

    fn read_unlock(&self) {
        self.flag.p();
        unsafe {
            *((&self.reader) as *const usize as *mut usize) -= 1;
        }
        if self.reader == 0 {
            self.lock.v();
        }
        self.flag.v();
    }

    fn write_lock(&self) {
        self.lock.p();
    }

    fn write_unlock(&self) {
        self.lock.v();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::Lock;
    use super::ParkLock;
    use super::SemLock;
    use super::SemRwLock;
    use super::SpinLock;

    const LOOP_SIZE: i32 = 50;

    #[test]
    fn spin() {
        let lock = Arc::new(SpinLock::new());
        single(lock.clone());
        two(lock.clone());
        multipal(lock.clone());
    }

    #[test]
    fn park() {
        let lock = Arc::new(ParkLock::new());
        single(lock.clone());
        two(lock.clone());
        multipal(lock.clone());
    }

    #[test]
    fn sem() {
        let lock = Arc::new(SemLock::new());
        single(lock.clone());
        two(lock.clone());
        multipal(lock.clone());
    }

    #[test]
    fn rwlock() {
        let lock = Arc::new(SemRwLock::new());
        let num = Arc::new(0);

        let lock_write = lock.clone();
        let num_write = num.clone();
        let write_thread = thread::spawn(move || {
            for _ in 0..LOOP_SIZE {
                thread::sleep(std::time::Duration::from_millis(1));
                lock_write.write_lock();
                unsafe {
                    *(num_write.as_ref() as *const i32 as *mut i32) += 1;
                }
                lock_write.write_unlock();
            }
        });
        let mut sub_threads = vec![];
        for _ in 0..10 {
            let lock_read = lock.clone();
            let num_read = num.clone();
            let read_thread = thread::spawn(move || {
                lock_read.read_lock();
                println!("{}", num_read);
                lock_read.read_unlock();
            });
            sub_threads.push(read_thread);
            thread::sleep(std::time::Duration::from_millis(5));
        }
        for t in sub_threads {
            t.join().unwrap();
        }
        write_thread.join().unwrap();
        assert_eq!(*num, LOOP_SIZE);
    }

    fn single(lock: Arc<dyn Lock>) {
        let mut num = 0;
        for _ in 0..LOOP_SIZE {
            lock.lock();
            num += 1;
            lock.unlock();
        }
        assert_eq!(num, LOOP_SIZE);
    }

    fn two(lock: Arc<dyn Lock>) {
        let num: Arc<i32> = Arc::new(0);

        let cloned_lock = lock.clone();
        let cloned_num: Arc<i32> = num.clone();
        let sub_thread = thread::spawn(move || {
            for _ in 0..LOOP_SIZE {
                cloned_lock.lock();
                unsafe {
                    *(cloned_num.as_ref() as *const i32 as *mut i32) += 1;
                }
                cloned_lock.unlock();
            }
        });

        for _ in 0..LOOP_SIZE {
            lock.lock();
            unsafe {
                *(num.as_ref() as *const i32 as *mut i32) += 1;
            }
            lock.unlock();
        }
        sub_thread.join().unwrap();
        assert_eq!(*num, LOOP_SIZE * 2);
    }

    fn multipal(lock: Arc<dyn Lock>) {
        let num: Arc<i32> = Arc::new(0);
        let sub_thread_size = 5;

        let mut sub_threads = vec![];
        for _ in 0..sub_thread_size {
            let cloned_lock = lock.clone();
            let cloned_num: Arc<i32> = num.clone();
            let t = thread::spawn(move || {
                for _ in 0..LOOP_SIZE {
                    cloned_lock.lock();
                    unsafe {
                        *(cloned_num.as_ref() as *const i32 as *mut i32) += 1;
                    }
                    cloned_lock.unlock();
                }
            });
            sub_threads.push(t);
        }

        for t in sub_threads {
            t.join().unwrap();
        }
        assert_eq!(*num, LOOP_SIZE * sub_thread_size);
    }
}
