use chrono::{DateTime, Local};

use crate::common::{
    error::{Error, ErrorKind},
    utils,
};

use super::strings::StringObject;

#[derive(Clone, Debug)]
pub enum ObjectValue {
    Null,
    Strings(StringObject),
}

#[derive(Clone, Debug)]
pub struct Object {
    value: ObjectValue,
    active_time: DateTime<Local>,
    expire_time: Option<DateTime<Local>>,
}

impl Object {
    // object method
    pub fn new(value: ObjectValue, expire_time: Option<&str>) -> Result<Self, Error> {
        let mut obj = Self {
            value,
            active_time: Local::now(),
            expire_time: None,
        };
        if let Some(time) = expire_time {
            obj.set_expire_time(time)?;
        }
        return Ok(obj);
    }

    pub fn get_type(&self) -> &str {
        return match &self.value {
            ObjectValue::Null => "Null",
            ObjectValue::Strings(_) => "String",
        };
    }

    pub fn get_encoding(&self) -> &str {
        return match &self.value {
            ObjectValue::Null => "Null",
            ObjectValue::Strings(s) => s.get_encoding(),
        };
    }

    pub fn idle_time(&self) -> u64 {
        return utils::elasped(self.active_time).num_milliseconds() as u64;
    }

    pub fn set_expire_time(&mut self, time: &str) -> Result<(), Error> {
        let expire_time = utils::parse_millis(time)?;
        if expire_time < Local::now() {
            return Err(Error::new(
                ErrorKind::Invalid,
                "expire time must greater than current time.",
            ));
        }
        self.expire_time = Some(expire_time);
        return Ok(());
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            return Local::now() > expire_time;
        }
        return false;
    }

    fn refresh_active_time(&mut self) {
        self.active_time = Local::now();
    }

    // string method
    pub fn get(&mut self) -> Option<Box<[u8]>> {
        self.refresh_active_time();
        return match &self.value {
            ObjectValue::Strings(s) => Some(s.get()),
            ObjectValue::Null => None,
        };
    }
}

#[test]
fn test_object() {
    use chrono::Duration;
    use std::thread;
    // base
    let mut obj = Object::new(ObjectValue::Null, None).unwrap();
    let sec = 2;
    let expire_time = Local::now()
        .checked_add_signed(Duration::seconds(sec))
        .unwrap();
    assert!(obj
        .set_expire_time(&expire_time.timestamp_millis().to_string())
        .is_ok());
    assert_eq!(obj.get_type(), "Null");
    assert_eq!(obj.get_encoding(), "Null");
    thread::sleep(std::time::Duration::from_secs(sec as u64));
    assert!(obj.is_expired());
}
