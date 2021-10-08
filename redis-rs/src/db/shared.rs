use std::{
    mem::{self, MaybeUninit},
    sync::Arc,
};

use crate::{common::utils, types::strings::StringObject};

pub const SHARED_NUMBER: i64 = 1000;

#[derive(Debug)]
pub struct SharedObject {
    integers: [Arc<StringObject>; SHARED_NUMBER as usize],
}

impl SharedObject {
    pub fn new() -> Self {
        let integers = Self::init_integers();
        return Self { integers };
    }

    pub fn get<T>(&self, val: T) -> Option<Arc<StringObject>>
    where
        T: AsRef<str>,
    {
        if let Some(num) = utils::parse_str::<T, i64>(&val) {
            return self.get_integer(num);
        } else {
            return None;
        }
    }

    pub fn get_integer(&self, num: i64) -> Option<Arc<StringObject>> {
        if num < SHARED_NUMBER {
            return Some(self.integers[num as usize].clone());
        } else {
            return None;
        }
    }

    fn init_integers() -> [Arc<StringObject>; SHARED_NUMBER as usize] {
        let mut uninit_integers: [MaybeUninit<Arc<StringObject>>; SHARED_NUMBER as usize] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..SHARED_NUMBER as usize {
            uninit_integers[i].write(Arc::new(StringObject::new(i.to_string())));
        }
        return unsafe {
            mem::transmute::<_, [Arc<StringObject>; SHARED_NUMBER as usize]>(uninit_integers)
        };
    }
}

#[test]
fn test_shared() {
    let obj = SharedObject::new();
    assert_eq!(obj.integers.len(), SHARED_NUMBER as usize);
    assert_eq!(obj.get_integer(0).unwrap().get().as_ref(), "0".as_bytes(),);
    assert_eq!(
        obj.get_integer(SHARED_NUMBER - 1).unwrap().get().as_ref(),
        "999".as_bytes(),
    );
}
