use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use super::cell::Cell;

// RcInner helps clone get a new Rc
#[derive(Debug)]
struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>, // Even thought inner in Rc is *mut pointer, but it not guarantee only one owner. So we use Cell packaging usize. (In fact, we Rc code promise only one can modify)
}

#[derive(Debug)]
struct Rc<T> {
    inner: NonNull<RcInner<T>>, // "Nonnull" in Rust is used instead of "*mut" to guarantee inner is not empty pointer. Like Option<T>
    _marker: PhantomData<RcInner<T>>, // TODO what it means?
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() }; // this is another reference to inner
        inner.ref_count.set(inner.ref_count.get() + 1);
        return Rc {
            inner: self.inner,
            _marker: PhantomData,
        };
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner: &RcInner<T> = unsafe { self.inner.as_ref() };
        let count = inner.ref_count.get();
        if count == 1 {
            drop(inner); // Drop the inner reference. FIXME Because Inner is a reference impl Copy trait. drop(inner) == drop(inner.clone()). So inner is not droped.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) }; // Drop inner of Rc after inner reference is droped. ("let _" make compiler knows that the Box is useless and can be droped)
        } else {
            inner.ref_count.set(count - 1);
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &unsafe { self.inner.as_ref() }.value;
    }
}

impl<T> Rc<T> {
    fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            ref_count: Cell::new(1),
        });
        Self {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) }, // convert Box to raw pointer. Prevent "inner" destroyed after function
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rc;

    #[test]
    fn set() {
        let s = String::from("hello");
        let r = Rc::new(s);
        assert_eq!(*r, "hello");
        let rc1 = r.clone();
        assert_eq!(*rc1, "hello");
    }

    #[test]
    fn drop() {
        let s = String::from("hello");
        let r = Rc::new(s);
        let rc1 = r.clone();
        assert_eq!(unsafe { r.inner.as_ref() }.ref_count.get(), 2);
        std::mem::drop(rc1);
        assert_eq!(unsafe { r.inner.as_ref() }.ref_count.get(), 1);
    }
}
