use crate::{common::utils, encoding::sds::Sds};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StringValue {
    Integer(i64),
    Raw(Sds),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringObject {
    value: StringValue,
}

impl Default for StringObject {
    fn default() -> Self {
        StringObject {
            value: StringValue::Integer(0_i64),
        }
    }
}

impl ToString for StringObject {
    fn to_string(&self) -> String {
        match &self.value {
            StringValue::Integer(i) => i.to_string(),
            StringValue::Raw(s) => s.to_string(),
        }
    }
}

impl StringObject {
    pub fn new<T>(value: T) -> Self
    where
        T: AsRef<str> + AsRef<[u8]>,
    {
        return StringObject {
            value: match utils::parse_str::<T, i64>(&value) {
                Some(i) => StringValue::Integer(i),
                _ => StringValue::Raw(Sds::new(value.as_ref())),
            },
        };
    }

    pub fn get_encoding(&self) -> &str {
        return match self.value {
            StringValue::Integer(_) => "Integer",
            StringValue::Raw(_) => "Raw",
        };
    }

    pub fn get(&self) -> Box<[u8]> {
        return match &self.value {
            StringValue::Integer(i) => i.to_string().as_bytes().into(),
            StringValue::Raw(s) => s.as_bytes().into(),
        };
    }
}

#[test]
fn test_strings() {
    // Raw
    let raw_str = "hello";
    let raw_obj = StringObject::new(raw_str);
    assert!(matches!(raw_obj.value, StringValue::Raw(_)));
    assert_eq!(raw_obj.get().as_ref(), raw_str.as_bytes());

    // Integer
    let integer_str = "1024";
    let integer_obj = StringObject::new(integer_str);
    assert!(matches!(integer_obj.value, StringValue::Integer(_)));
    assert_eq!(integer_obj.get().as_ref(), integer_str.as_bytes());
}
