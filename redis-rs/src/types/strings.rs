use crate::encoding::{encoding::Encoding, sds::Sds};

use super::object::{EncodingType, Object, RedisType};

pub struct StringObject<T>
where
    T: Encoding,
{
    redis_type: RedisType,
    encoding_type: EncodingType,
    val: Box<T>,
}

impl Default for StringObject<i32> {
    fn default() -> Self {
        StringObject {
            redis_type: RedisType::RedisString,
            encoding_type: EncodingType::Number,
            val: Box::new(0_i32),
        }
    }
}

impl Object<T> for StringObject<T> {
    fn create_object(value: &[char]) -> Box<dyn Object<T>> {
        StringObject {
            redis_type: RedisType::RedisString,
            encoding_type: EncodingType::Raw,
            val: Box::new(0 as dyn Encoding),
        }
    }
}
