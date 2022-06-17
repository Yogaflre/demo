/*
 * 62进制
 * 常用于生辰短链接
 * 假设反馈最长10位结果，则一共可以获取62^10个组合方式
 */

// 用于编码的62个字符
const SCALE: usize = 62;
const CHARS: [u8; SCALE] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];
struct Base62;
impl Base62 {
    fn encode_num(mut num: usize) -> String {
        let mut res: Vec<u8> = vec![];
        while num > SCALE {
            res.push(CHARS[num % SCALE]);
            num /= SCALE;
        }
        res.push(CHARS[num]);
        res.reverse();
        return String::from_utf8(res).expect("Base62 encode error");
    }

    fn decode_to_num(string: String) -> usize {
        let mut res = 0;
        for (i, b) in string.bytes().rev().enumerate() {
            // 使用二分查找寻找字符的索引位置
            let tmp = CHARS.binary_search(&b).expect("Base62 decode error");
            res += SCALE.pow(i as u32) * tmp;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Base62::decode_to_num(Base62::encode_num(0)), 0);
        assert_eq!(Base62::decode_to_num(Base62::encode_num(1)), 1);
        assert_eq!(Base62::decode_to_num(Base62::encode_num(64)), 64);
        assert_eq!(
            Base62::decode_to_num(Base62::encode_num(114219942)),
            114219942
        )
    }
}
