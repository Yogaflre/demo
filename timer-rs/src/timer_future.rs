use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct ShareState {
    completed: bool,
    waker: Option<Waker>,
}
struct TimerFuture {
    share_state: Arc<Mutex<ShareState>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.share_state.lock().unwrap();
        if state.completed {
            return Poll::Ready(());
        } else {
            state.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
    }
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        let share_state = Arc::new(Mutex::new(ShareState {
            completed: false,
            waker: None,
        }));
        let thread_share_state = share_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = thread_share_state.lock().unwrap();
            state.completed = true;
            if let Some(w) = state.waker.take() {
                w.wake();
            }
        });
        return TimerFuture { share_state };
    }
}

#[test]
fn timer_future() {
    let timer = TimerFuture::new(Duration::from_secs(2));
}
