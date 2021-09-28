use std::sync::Arc;

use chrono::{DateTime, Local};

use crate::{
    common::{
        error::{Error, ErrorKind},
        utils,
    },
    encoding::sds::Sds,
    types::{
        dict::Dict,
        object::{Object, ObjectValue},
        strings::StringObject,
    },
};

use super::shared::SharedObject;

const THRESH_HOLD: f32 = 0.9;

struct Db {
    id: u8,
    dict: Dict<Object>,
    expires: Dict<DateTime<Local>>,
    shared_object: SharedObject,
}

impl Db {
    fn new(id: u8) -> Self {
        Self {
            id,
            dict: Dict::new(THRESH_HOLD),
            expires: Dict::new(THRESH_HOLD),
            shared_object: SharedObject::new(),
        }
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

    pub fn set_expire_time(&mut self, key: Arc<Sds>, time: &str) -> Result<(), Error> {
        let expire_time: DateTime<Local> = utils::parse_millis(time)?;
        if expire_time < Local::now() {
            return Err(Error::new(
                ErrorKind::Invalid,
                "expire time must greater than current time.",
            ));
        }
        self.expires.dict_add(key, expire_time);
        return Ok(());
    }

    pub fn is_expired(&mut self, key: Arc<Sds>) -> Result<bool, Error> {
        if let Some(expire_time) = self.expires.dict_get(key)? {
            return Ok(Local::now() > expire_time);
        }
        return Ok(false);
    }

    pub fn set<T>(&mut self, key: T, val: T) -> Result<bool, Error>
    where
        T: AsRef<[u8]> + AsRef<str>,
    {
        //TODO 条件
        let value = match self.shared_object.get(&val) {
            Some(o) => o,
            None => Arc::new(StringObject::new(val)),
        };
        return self.dict.dict_add(
            Arc::new(Sds::new(key.as_ref())),
            Object::new(ObjectValue::Strings(value))?,
        );
    }
}

#[test]
fn test_db() {
    let mut db = Db::new(0);
    assert!(db.set("1", "1").is_ok());
    // base
    assert_eq!(db.exists(&["1", "2"]).unwrap().as_ref(), [true, false]);
}
