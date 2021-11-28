use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

struct StepAddFuture {
    shared: Arc<Mutex<StepAddShared>>,
}

struct StepAddShared {
    target: usize,
    add: usize,
    waker: Option<Waker>,
}

impl Future for StepAddFuture {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut shared = self.shared.lock().unwrap();
        if shared.add == 0 {
            println!("Ready!");
            return Poll::Ready(shared.target);
        } else {
            shared.waker = Some(cx.waker().clone());
            println!("Pending...");
            return Poll::Pending;
        }
    }
}

impl StepAddFuture {
    fn new(target: usize, add: usize) -> Self {
        let shared = Arc::new(Mutex::new(StepAddShared {
            target,
            add,
            waker: None,
        }));
        let shared_clone = shared.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            let mut shared_locked = shared_clone.lock().unwrap();
            shared_locked.target += shared_locked.add;
            shared_locked.add = 0;
            if let Some(waker) = &shared_locked.waker {
                waker.wake_by_ref();
            }
        });
        return StepAddFuture { shared };
    }
}
