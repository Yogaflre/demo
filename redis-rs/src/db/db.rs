use crate::{
    common::error::Error,
    encoding::sds::Sds,
    types::{
        dict::Dict,
        object::{Object, ObjectValue},
        strings::StringObject,
    },
};

struct Db {
    id: u8,
    dict: Dict<Object>,
}

impl Db {
    fn new(id: u8) -> Self {
        Self {
            id,
            dict: Dict::new(0.9),
        }
    }

    pub fn set<T>(&mut self, key: T, val: T) -> Result<bool, Error>
    where
        T: AsRef<[u8]> + AsRef<str>,
    {
        //TODO 缓存、条件
        return self.dict.dict_add(
            Sds::new(key.as_ref()),
            Object::new(ObjectValue::Strings(StringObject::new(val)), None)?,
        );
    }

    pub fn exists<T>(&mut self, keys: &[T]) -> Result<Box<[bool]>, Error>
    where
        T: AsRef<[u8]>,
    {
        return Ok(keys
            .iter()
            .map(|k| Sds::new(k.as_ref()))
            .map(|s| self.dict.dict_contanins_key(&s).unwrap())
            .collect::<Box<[bool]>>());
    }
}

#[test]
fn test_db() {
    let mut db = Db::new(0);
    assert!(db.set("1", "1").is_ok());
    assert_eq!(db.exists(&["1", "2"]).unwrap().as_ref(), [true, false]);
}
