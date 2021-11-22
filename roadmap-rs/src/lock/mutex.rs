use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            v: UnsafeCell::new(v),
        }
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        /*
         * Error1: Codes may be reordered.
         *
         * Fix Error1: Ordering::Acquire ensure no reads/writes could reordered BEFORE this line.
         * Fix Error2: Ordering::Acquire ensure ALL WRITES in other threads that release the same atomic variable are visible in the current thread.
         *
         * Ordering::AcqRel means load by Acquire and store by Release.
         */
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed) // If use compare_exchange, only fail when current value is incorrect.
            .is_err()
        {
            // NOTE Avoid cost through yield.
            while self.locked.load(Ordering::Relaxed) == true {
                std::thread::yield_now(); // Yield when locked is true.
            }
            std::thread::yield_now(); // Yield when locked is false, but cpu set true is failed.
        }
        /*
         * Error1: Codes may be reordered.
         * self.locked.store(false, Ordering::Relaxed);
         * let r = f(unsafe { &mut *self.v.get() });
         *
         * Error2: store(value, Relaxed) can not guarantee we stored value is visable to other threads.
         *
         * Fix Error1: Ordering::Release ensure no reads/writes could reordered AFTER this line.
         * Fix Error2: Ordering::Release ensure ALL WRITES in the current thread are visible in other threads that acquire the same atomic variable.
         */
        let r = f(unsafe { &mut *self.v.get() });
        self.locked.store(false, Ordering::Release);
        return r;
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            atomic::{AtomicBool, AtomicU8, Ordering},
            Arc,
        },
        thread,
    };

    use super::Mutex;

    #[test]
    fn race() {
        let m = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let mc = m.clone();
                return thread::spawn(move || {
                    for _ in 0..100 {
                        mc.with_lock(|v| *v += 1);
                    }
                });
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(m.with_lock(|v| *v), 10 * 100);
    }

    #[test]
    fn relaxed() {
        let x: &'static AtomicU8 = Box::leak(Box::new(AtomicU8::new(0)));
        let y: &'static AtomicU8 = Box::leak(Box::new(AtomicU8::new(0)));
        let t1 = thread::spawn(|| {
            let x1 = x.load(Ordering::Relaxed);
            y.store(x1, Ordering::Relaxed);
            return x1;
        });
        let t2 = thread::spawn(|| {
            let y1 = y.load(Ordering::Relaxed);
            x.store(7, Ordering::Relaxed);
            return y1;
        });
        /*
         * NOTE What will happend is "t1 == t2 == 7". Because:
         *  1. Cpu/Compiler could to resort commands if they have not dependences.
         */
        println!("{}", t1.join().unwrap());
        println!("{}", t2.join().unwrap());
    }

    #[test]
    fn seqcst() {
        let x = Box::leak(Box::new(AtomicBool::new(false)));
        let y = Box::leak(Box::new(AtomicBool::new(false)));
        let z = Box::leak(Box::new(AtomicU8::new(0)));

        /*
         * Fix Error3: Use SeqCst. All threads see all sequentially consistent operations in the same order.
         */
        let _tx = thread::spawn(|| x.store(true, Ordering::SeqCst));
        let _ty = thread::spawn(|| y.store(true, Ordering::SeqCst));

        let t1 = thread::spawn(|| {
            while !x.load(Ordering::SeqCst) {}
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        let t2 = thread::spawn(|| {
            while !y.load(Ordering::SeqCst) {}
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        /*
         * Error3: z could be 0. Because x & tx and y & ty do not have happends-before
         * relationship.
         */
        println!("{}", z.load(Ordering::SeqCst));
    }
}
