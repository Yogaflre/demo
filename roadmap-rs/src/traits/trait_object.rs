use std::{
    any::{Any, TypeId},
    ptr::NonNull,
};

/*
 * TODO Need more infomation from Trait.
 */
struct VTable {
    type_id: TypeId, // specific type id
    drop: fn(*mut ()),
}

struct TraitObj {
    p: *mut (),
    vtable: VTable,
}

impl TraitObj {
    fn new<T>(v: T) -> Self
    where
        T: Any,
    {
        Self {
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(v))) }.as_ptr() as *mut (),
            vtable: VTable {
                type_id: TypeId::of::<T>(),
                drop: |p: *mut ()| drop(unsafe { Box::from_raw(p as *mut T) }),
            },
        }
    }
}

impl Drop for TraitObj {
    fn drop(&mut self) {
        (self.vtable.drop)(self.p);
    }
}

#[cfg(test)]
mod tests {
    use super::TraitObj;
    trait Hello {
        fn hello();
    }

    #[test]
    fn drop_test() {
        let n = Box::new(123);
        let p: *const i32 = &*n;
        let obj = TraitObj::new(n);
        drop(obj);
        assert_ne!(unsafe { *p }, 123);
    }
}
