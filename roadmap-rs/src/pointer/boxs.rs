use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/*
 * <Drop check>
 *
 * If we impl Drop trait, the compiler will assume we access the "&mut T" in drop()
 */
struct Boxs<T> {
    /*
     * Error3: If we use *mut T, The Boxs will be invariance
     * FIX Error3: Use NonNull<T> instead of raw pointer
     */
    p: NonNull<T>,
    /*
     * FIX Error2
     * Tell the compiler that we own T even though "p" is a pointer of T. So compiler will check
     * all fields drop()
     */
    mark: PhantomData<T>,
}

/*
 * FIX Error1: "unsafe impl<#[may_dangle] T> Drop for Boxs<T> { ... }" (Unstable)
 *
 * Use "#[may_dangle]" to assure the compiler that we don't access the T in drop(&mut self)
 */
impl<T> Drop for Boxs<T> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p.as_ptr()) };
    }
}

impl<T> Deref for Boxs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boxs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.p.as_mut() }
    }
}

impl<T> Boxs<T> {
    fn new(value: T) -> Self {
        Self {
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(value))) },
            mark: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Boxs;
    use std::{fmt::Debug, iter::Empty};

    #[test]
    fn bad1() {
        let mut n = 0;
        let b = Boxs::new(&mut n);
        /*
         * Error1: This is not compile (This situation only occurs when T is a reference and Boxs impl Drop trait)
         * drop(b) will called at the "bad()" function end, so we can't access "&n".
         *
         * Because Compiler doesn't know whether "&mut n" is accessed in "b"'s drop()
         */
        // println!("{:?}", n);
    }

    struct Demo<T: Debug>(T);
    impl<T: Debug> Drop for Demo<T> {
        fn drop(&mut self) {
            println!("{:?}", self.0); // access T by &mut T
        }
    }
    #[test]
    fn bad2() {
        let mut n = 0;
        let b = Boxs::new(Demo(&mut n));
        /*
         * Error2: If we fixed Error1, this line should not be compiled pass, but it did.
         *
         * Because Compiler know we doesn't access T, but it doesn't know whether we own T (Boxs.p
         * is a pointer of T) and drop T
         */
        // println!("{:?}", n);
    }

    #[test]
    fn covariance() {
        let s = String::from("hello");
        let mut b1: Boxs<&'_ str> = Boxs::new(&*s);
        let b2: Boxs<&'static str> = Boxs::new("hi"); // &'static: &'_
        b1 = b2;
    }

    #[test]
    fn lifetime() {
        let mut num: i32 = 123;
        let mut iter: Empty<Demo<&'static mut i32>> = Empty::default();
        {
            let mut demo: Option<Demo<&'_ mut i32>> = Some(Demo(&mut num));
            demo = iter.next(); // "&mut num" is droped here.
        }
        println!("{:?}", num);
        let _ = iter.next();
    }
}
