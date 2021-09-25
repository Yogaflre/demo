use std::hash::Hash;

const MB_SIZE: usize = 1024;

/*
 * Simple Dynamic String
 */
#[derive(Default, Debug, Eq, Clone)]
pub struct Sds {
    used: usize,
    free: usize,
    buf: Box<[u8]>,
}

impl PartialEq for Sds {
    fn eq(&self, other: &Self) -> bool {
        return self.buf[..self.used] == other.buf[..other.used];
    }
}

impl Hash for Sds {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}

impl ToString for Sds {
    fn to_string(&self) -> String {
        return std::str::from_utf8(&self.buf[..self.used])
            .map(|s| s.to_string())
            .unwrap();
    }
}

impl Sds {
    pub fn new(value: &[u8]) -> Self {
        let used = value.len();
        Self {
            used,
            free: 0,
            buf: value.into(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.buf[..self.used];
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
    pub fn push(&mut self, value: &[u8]) {
        if self.free < value.len() {
            let need_length = self.used + value.len();
            let mut buf;
            if need_length > MB_SIZE {
                buf = Self::malloc(need_length + MB_SIZE);
            } else {
                buf = Self::malloc(need_length << 1);
            }
            Self::copy(&self.buf, &mut buf, 0);
            self.buf = buf;
        }
        Self::copy(&value, &mut self.buf, self.used);
        self.set_size(self.used + value.len());
    }

    fn copy(from: &[u8], to: &mut [u8], to_index: usize) {
        to[to_index..].copy_from_slice(from);
    }

    fn malloc(size: usize) -> Box<[u8]> {
        return Vec::with_capacity(size).into_boxed_slice();
    }

    fn set_size(&mut self, used: usize) {
        self.used = used;
        self.free = self.buf.len() - used;
    }
}
