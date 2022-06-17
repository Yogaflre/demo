// <位图>

// 给定1000万个整数，数值范围在1～1亿之间，如何快速查找某个整数是否在这些整数之间？
#[derive(Debug)]
struct BitMap {
    // 使用u8 8bits当作一个基本单位
    bytes: Vec<u8>,
    // 一共可包含多少个bits位
    nbits: usize,
    // 和bytes中每个元素大小保持一致
    divid: usize,
}

impl BitMap {
    pub fn new(nbits: usize) -> BitMap {
        BitMap {
            nbits,
            bytes: vec![0; (nbits / 8) + 1],
            divid: 8,
        }
    }

    fn set(&mut self, n: usize) {
        if n < self.nbits {
            // 计算在u8中的位置
            let remainder = n % self.divid;
            // 计算bytes中的索引位置
            let index = n / self.divid;
            // 将新加的数和原始值做或运算
            self.bytes[index] |= 1 << remainder;
        }
    }

    fn get(&self, n: usize) -> bool {
        if n > self.nbits {
            return false;
        } else {
            let remainder = n % self.divid;
            let index = n / self.divid;
            // 计算现有bytes中是否包含n(与运算)
            return (self.bytes[index] & (1 << remainder)) != 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut bit_map = BitMap::new(16);

        bit_map.set(1);
        bit_map.set(8);
        bit_map.set(14);

        assert_eq!(bit_map.get(1), true);
        assert_eq!(bit_map.get(8), true);
        assert_eq!(bit_map.get(14), true);
        assert_eq!(bit_map.get(16), false);
    }
}
