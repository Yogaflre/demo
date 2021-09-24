use crate::encoding::{encoding::Encoding, sds::Sds};

use super::object::{EncodingType, Object, RedisType};

pub struct StringObject {
    redis_type: RedisType,
    encoding_type: EncodingType,
    val: Box<dyn Encoding>,
}

impl Default for StringObject {
    fn default() -> Self {
        StringObject {
            redis_type: RedisType::RedisString,
            encoding_type: EncodingType::Number,
            val: Box::new(0_i32),
        }
    }
}

impl Object for StringObject {
    fn create_object(value: &[char]) -> Box<Self> {
        return Box::new(StringObject {
            redis_type: RedisType::RedisString,
            encoding_type: EncodingType::Number,
            val: Box::new(0_i32) as Box<dyn Encoding>,
        });
    }
}
