use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    common::error::{Error, ErrorKind, Result},
    encoding::sds::Sds,
    types::{
        dict::Dict,
        object::{Object, ObjectValue},
        strings::StringObject,
    },
};

use super::{rdb, shared::SharedObject};

const THRESH_HOLD: f32 = 0.9;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Db {
    pub store_dir: String,
    pub dict: Dict<Object>,
    pub expires: Dict<i64>,
    #[serde(skip)]
    shared_object: SharedObject,
    #[serde(skip)]
    is_saving: Arc<AtomicBool>,
}

impl Db {
    pub fn new(store_dir: String, dict: Option<Dict<Object>>, expires: Option<Dict<i64>>) -> Self {
        return Self {
            store_dir,
            dict: dict.unwrap_or(Dict::new(THRESH_HOLD)),
            expires: expires.unwrap_or(Dict::new(THRESH_HOLD)),
            shared_object: SharedObject::default(),
            is_saving: Arc::new(AtomicBool::default()),
        };
    }

    pub fn exist(&mut self, key: &str) -> Result<bool> {
        return match self.check_exist(Arc::new(key.into())) {
            Ok(true) => Ok(true),
            _ => Ok(false),
        };
    }

    pub fn delete(&mut self, key: &str) -> Result<bool> {
        let k: Arc<Sds> = Arc::new(key.into());

        self.dict.dict_delete(k.clone())?;
        self.expires.dict_delete(k)?;
        return Ok(true);
    }

    pub fn set_expire_time(&mut self, key: &str, time: &str) -> Result<()> {
        let k: Arc<Sds> = Arc::new(key.into());
        self.check_exist(k.clone())?;

        let expire_time: i64 = time
            .parse::<i64>()
            .map_err(|e| Error::new(ErrorKind::Parser, e.to_string()))?;
        if expire_time < Local::now().timestamp_millis() {
            return Err(Error::new(
                ErrorKind::Invalid,
                "expire time must greater than current time.".to_string(),
            ));
        }
        self.expires.dict_add(k, expire_time)?;
        return Ok(());
    }

    pub fn is_expired(&mut self, key: &str) -> Result<bool> {
        return self.check_exist(Arc::new(key.into())).map(|e| !e);
    }

    pub fn delete_expire(&mut self, key: &str) -> Result<bool> {
        let k: Arc<Sds> = Arc::new(key.into());
        self.check_exist(k.clone())?;

        self.expires.dict_delete(k)?;
        return Ok(true);
    }

    pub fn set(&mut self, key: &str, val: &str) -> Result<bool> {
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

    pub fn get(&mut self, key: &str) -> Result<Option<Box<[u8]>>> {
        let k: Arc<Sds> = Arc::new(key.into());

        return match self.check_exist(k.clone()) {
            Ok(true) => Ok(self.dict.dict_get(k)?.and_then(|mut o| o.get())),
            _ => Ok(None),
        };
    }

    pub fn bgsave(&self) {
        if self
            .is_saving
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            let db_clone = self.clone();
            let is_saving_clone = self.is_saving.clone();
            thread::spawn(move || {
                rdb::save(db_clone, is_saving_clone);
            });
        }
    }

    fn check_exist(&mut self, key: Arc<Sds>) -> Result<bool> {
        if !self.dict.dict_contanins_key(key.clone())? {
            return Err(Error::new(
                ErrorKind::Invalid,
                format!("key is not exist: {}", key.to_string()),
            ));
        }

        let expired;
        if let Some(expire_time) = self.expires.dict_get(key.clone())? {
            expired = Local::now().timestamp_millis() > expire_time;
        } else {
            expired = false;
        }

        if !expired {
            return Ok(true);
        }

        // FIXME master才做过期删除操作
        self.delete(&key.to_string())?;

        return Ok(false);
    }
}

#[test]
fn db() {
    let mut db = Db::new("store".to_string(), None, None);
    let kv = "default";
    assert!(db.set(kv, kv).is_ok());
    // base
    assert!(db.exist(kv).unwrap());
    assert_eq!(db.get(kv).unwrap().unwrap().as_ref(), kv.as_bytes());
    assert!(db.delete(kv).unwrap());
    // expire
    let expire_time = Local::now().timestamp_millis() + 1000;
    assert!(db.set_expire_time(kv, &expire_time.to_string()).is_err());
    assert!(db.set(kv, kv).is_ok());
    assert!(db.set_expire_time(kv, &expire_time.to_string()).is_ok());
    assert_eq!(db.get(kv).unwrap().unwrap().as_ref(), kv.as_bytes());
    thread::sleep(std::time::Duration::from_secs(1));
    assert!(db.is_expired(kv).unwrap());
    assert!(!db.exist(kv).unwrap());
    assert_eq!(db.get(kv).unwrap(), None);
}

#[test]
fn bgsave() {
    let kv = "default";
    let base_path = "store";
    let rdb_path = std::path::Path::new("store/db.rdb");
    if rdb_path.exists() {
        let db = rdb::load(base_path).unwrap();
        assert!(db.is_some());
        assert_eq!(
            db.unwrap().get(kv).unwrap().unwrap().as_ref(),
            kv.as_bytes()
        );
    } else {
        let mut db = Db::new(base_path.to_string(), None, None);
        assert!(db.set(kv, kv).is_ok());
        db.set(kv, kv).unwrap();
        db.bgsave();
        thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(db.is_saving.load(Ordering::SeqCst), false);
    }
}
