use crate::encoding::encoding::Encoding;

#[derive(Clone)]
pub enum RedisType {
    RedisString,
    RedisHash,
}

#[derive(Clone)]
pub enum EncodingType {
    Number,
    Raw,
    Dict,
}

pub trait Object<T>
where
    T: Encoding,
{
    fn create_object(value: &[char]) -> Box<dyn Object<T>>;
}
