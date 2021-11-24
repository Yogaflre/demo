use std::{
    any::{Any, TypeId},
    ptr::NonNull,
};

struct VTable {
    type_id: TypeId, // specific type id
    drop: fn(*mut ()),
}

/*
 * TODO We need more infomation from Trait.
 * Generic must be Sized, but dyn Trait already is a Trait object. How do we get metadata from
 * Trait?
 */
struct TraitObj /*<T> T is Trait?*/ {
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

    fn cast_ref<T>(&self) -> Result<&T, ()>
    where
        T: Any,
    {
        if self.vtable.type_id == TypeId::of::<T>() {
            return Ok(unsafe { &*(self.p as *mut T) });
        } else {
            return Err(());
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

    #[test]
    fn drop_test() {
        let n = Box::new(123);
        let p: *const i32 = &*n;
        let obj = TraitObj::new(n);
        drop(obj);
        assert_ne!(unsafe { *p }, 123);
    }

    #[test]
    fn cast() {
        let s = TraitObj::new(String::from("hello"));
        let s_ref = s.cast_ref::<String>().unwrap();
        assert_eq!(s_ref.len(), 5);
        let v = TraitObj::new(vec![1, 2, 3]);
        let v_ref = v.cast_ref::<Vec<i32>>().unwrap();
        assert_eq!(v_ref[1], 2);
        let v_err = v.cast_ref::<&str>();
        assert_eq!(v_err, Err(()));
    }
}
