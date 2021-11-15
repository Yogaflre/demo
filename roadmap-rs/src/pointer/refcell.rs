use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use super::cell::Cell;

#[derive(Clone, Copy, Debug)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

#[derive(Debug)]
struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// Ref & RefMut is used to implement Drop trait
struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<Ref<T>> {
        return match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                return Some(Ref { refcell: self });
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                return Some(Ref { refcell: self });
            }
            RefState::Exclusive => None,
        };
    }

    fn borrow_mut(&self) -> Option<RefMut<T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            return Some(RefMut { refcell: self });
        } else {
            return None;
        }
    }
}

// Update refcell state when reference is droped
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
            _ => unreachable!(),
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
            _ => unreachable!(),
        }
    }
}

// provide method to get inner T
impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return unsafe { &(*self.refcell.value.get()) };
    }
}

// DerefMut needs Deref before
impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return unsafe { &*self.refcell.value.get() };
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return unsafe { &mut *self.refcell.value.get() };
    }
}

#[cfg(test)]
mod tests {
    use super::RefCell;

    #[test]
    fn borrow() {
        let cell = RefCell::new(String::new());
        let b1 = cell.borrow();
        let b2 = cell.borrow();
        assert!(b1.is_some());
        assert!(b2.is_some());
        let bm1 = cell.borrow_mut();
        assert!(bm1.is_none());
    }

    #[test]
    fn borrow_mut() {
        let cell = RefCell::new(String::new());
        let bm1 = cell.borrow_mut();
        assert!(bm1.is_some());
        let b1 = cell.borrow();
        assert!(b1.is_none());
    }

    #[test]
    fn drop() {
        let cell = RefCell::new(String::from("hello"));
        let b1 = cell.borrow();
        let b2 = cell.borrow();
        assert_eq!(**(b2.as_ref().unwrap()), "hello");
        std::mem::drop(b1);
        assert!(cell.borrow_mut().is_none());
        std::mem::drop(b2);
        let bm1 = cell.borrow_mut();
        assert_eq!(**(bm1.as_ref().unwrap()), "hello");
    }
}
