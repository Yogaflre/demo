use std::cell::UnsafeCell;

use super::cell::Cell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<&T> {
        return match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                return Some(unsafe { &mut *self.value.get() });
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                return Some(unsafe { &mut *self.value.get() });
            }
            RefState::Exclusive => None,
        };
    }

    fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            return Some(unsafe { &mut *self.value.get() });
        } else {
            return None;
        }
    }
}
