use std::sync::Arc;

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::common::{error::Error, utils};

use super::strings::StringObject;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectValue {
    Null,
    Strings(Arc<StringObject>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    value: ObjectValue,
    active_time: i64,
}

impl Object {
    // object method
    pub fn new(value: ObjectValue) -> Result<Self, Error> {
        let obj = Self {
            value,
            active_time: Local::now().timestamp_millis(),
        };
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

    pub fn idle_time(&self) -> i64 {
        return utils::elasped(self.active_time);
    }

    fn refresh_active_time(&mut self) {
        self.active_time = Local::now().timestamp_millis();
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
    // base
    let mut obj = Object::new(ObjectValue::Null).unwrap();
    assert_eq!(obj.get_type(), "Null");
    assert_eq!(obj.get_encoding(), "Null");
}
