use std::{
    mem,
    sync::Arc,
    task::{RawWaker, RawWakerVTable, Waker},
    thread::{self, Thread},
};

#[derive(Clone)]
pub struct MyWaker {
    t: Thread,
}

impl MyWaker {
    pub fn new() -> Self {
        Self {
            t: thread::current(),
        }
    }

    pub fn convert(self) -> Waker {
        let mywaker_arc = Arc::into_raw(Arc::new(self)) as *const MyWaker;
        let raw_waker = RawWaker::new(mywaker_arc as *const (), &VTABLE);
        return unsafe { Waker::from_raw(raw_waker) };
    }
}

fn my_wake(waker: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(waker) };
    waker_arc.t.unpark();
}

fn my_wake_ref(waker: &MyWaker) {
    unsafe { (*(waker as *const MyWaker)).t.unpark() };
}

fn my_clone(waker: &MyWaker) -> RawWaker {
    let waker_arc = unsafe { Arc::from_raw(waker) };
    mem::forget(waker_arc.clone());
    return RawWaker::new(Arc::into_raw(waker_arc) as *const (), &VTABLE);
}

fn my_drop(waker: &MyWaker) {
    drop(unsafe { Arc::from_raw(waker) });
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |w| my_clone(&*(w as *const MyWaker)),
        |w| my_wake(&*(w as *const MyWaker)),
        |w| my_wake_ref(&*(w as *const MyWaker)),
        |w| my_drop(&*(w as *const MyWaker)),
    )
};
