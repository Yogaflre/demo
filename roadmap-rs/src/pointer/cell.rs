use std::cell::UnsafeCell;

/*
 * 1.Not implement Sync trait, only one thread can hold reference of Cell
 * 2.value T implement Copy trait, no one can get reference out of Cell
 */
pub struct Cell<T>
where
    T: Copy, // not get reference out Cell
{
    value: UnsafeCell<T>, //TODO Can not use raw pointer instead of UnsafeCell. But reason is .. ?
}

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T {
        return unsafe { *self.value.get() };
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn set() {
        let cell = Cell::new(0);
        assert_eq!(cell.get(), 0);
        cell.set(1);
        assert_eq!(cell.get(), 1);
    }
}
