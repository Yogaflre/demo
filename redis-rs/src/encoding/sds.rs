use std::hash::Hash;

use super::encoding::Encoding;
const MB_SIZE: usize = 1024;

/*
 * Simple Dynamic String
 */
#[derive(Default, Debug, Eq, Clone)]
pub struct Sds {
    used: usize,
    free: usize,
    buf: Box<[char]>,
}

impl Encoding for Sds {}

impl PartialEq for Sds {
    fn eq(&self, other: &Self) -> bool {
        return self.buf == other.buf;
    }
}

impl Hash for Sds {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}

impl ToString for Sds {
    fn to_string(&self) -> String {
        return self.buf[..self.used].iter().collect::<String>();
    }
}

impl Sds {
    pub fn new(value: &[char]) -> Self {
        let used = value.len();
        Self {
            used,
            free: 0,
            buf: value.into(),
        }
    }

    /*
     * 利用SDS属性快速获取字符串长度
     */
    pub fn len(&self) -> usize {
        return self.used;
    }

    /*
     * 动态扩容
     */
    pub fn push(&mut self, value: &[char]) {
        if self.free < value.len() {
            let need_length = self.used + value.len();
            let mut buf;
            if need_length > MB_SIZE {
                buf = Self::malloc(need_length + MB_SIZE);
            } else {
                buf = Self::malloc(need_length << 2);
            }
            Self::copy(&self.buf, &mut buf, 0);
            self.buf = buf;
        }
        Self::copy(value, &mut self.buf, 0);
        self.set_size(self.used + value.len());
    }

    fn copy(from: &[char], to: &mut [char], to_index: usize) {
        to[to_index..].copy_from_slice(from);
    }

    fn malloc(size: usize) -> Box<[char]> {
        return Vec::with_capacity(size).into();
    }

    fn set_size(&mut self, used: usize) {
        self.used = used;
        self.free = self.buf.len() - used;
    }
}
