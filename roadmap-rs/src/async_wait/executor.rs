use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
};

use super::waker::MyWaker;

pub struct Executor;

impl Executor {
    pub fn block_on<F: Future>(mut f: F) -> F::Output {
        let waker = MyWaker::new().convert();
        let mut context = Context::from_waker(&waker);
        let mut future: Pin<&mut F> = unsafe { Pin::new_unchecked(&mut f) };

        loop {
            match Future::poll(future.as_mut(), &mut context) {
                Poll::Ready(val) => return val,
                Poll::Pending => thread::park(),
            };
        }
    }
}
