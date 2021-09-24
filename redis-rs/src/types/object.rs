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

pub trait Object {
    fn create_object(value: &[char]) -> Box<Self>
    where
        Self: Sized;
}
