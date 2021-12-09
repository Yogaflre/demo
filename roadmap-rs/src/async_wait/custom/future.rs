use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
};

use super::reactor::{Reactor, TaskState};

#[derive(Clone)]
pub struct Task {
    id: usize,
    data: u64,
    reactor: Arc<Mutex<Reactor>>,
}

impl Future for Task {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut reactor = self.reactor.lock().unwrap();

        if reactor.is_ready(self.id) {
            reactor
                .tasks
                .get_mut(&self.id)
                .map(|state| *state = TaskState::Finished)
                .unwrap();
            return Poll::Ready(self.id);
        } else if reactor.tasks.contains_key(&self.id) {
            reactor
                .tasks
                .insert(self.id, TaskState::NotReady(cx.waker().clone()))
                .unwrap();
            return Poll::Pending;
        } else {
            reactor.register(self.id, self.data, cx.waker().clone());
            return Poll::Pending;
        }
    }
}

impl Task {
    pub fn new(id: usize, data: u64, reactor: Arc<Mutex<Reactor>>) -> Self {
        Self { id, data, reactor }
    }
}
