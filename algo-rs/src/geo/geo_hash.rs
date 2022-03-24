enum GeoAxis {
    Lon,
    Lat,
}
impl GeoAxis {
    fn get_range(&self) -> (f32, f32) {
        match self {
            Self::Lon => (-180.0, 180.0),
            Self::Lat => (-90.0, 90.0),
        }
    }
}
enum GeoLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Level10,
}
impl GeoLevel {
    fn get_level_range(&self) -> usize {
        match self {
            Self::Level1 => 3,
            Self::Level2 => 5,
            Self::Level3 => 8,
            Self::Level4 => 10,
            Self::Level5 => 13,
            Self::Level6 => 15,
            Self::Level7 => 18,
            Self::Level8 => 20,
            Self::Level9 => 23,
            Self::Level10 => 25,
        }
    }

    fn get_level(&self) -> usize {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
            Self::Level5 => 5,
            Self::Level6 => 6,
            Self::Level7 => 7,
            Self::Level8 => 8,
            Self::Level9 => 9,
            Self::Level10 => 10,
        }
    }
}

struct GeoHash;
impl GeoHash {
    pub fn get_geo_hash(lon: f32, lat: f32, level: GeoLevel) -> String {
        let lo = Self::convert_bits(lon, GeoAxis::Lon, &level);
        let la = Self::convert_bits(lat, GeoAxis::Lat, &level);

        let bits = Self::merge_bis(lo, la);

        return Self::base32(bits, level);
    }

    /*
     * convert lon/lat to range 32 bits
     * binary search level range.
     */
    fn convert_bits(num: f32, axis: GeoAxis, level: &GeoLevel) -> u32 {
        let (mut l, mut r) = axis.get_range();
        let mut res: u32 = 0;
        let mut tmp: u32 = i32::min_value() as u32;
        for _ in 0..level.get_level_range() {
            let mid: f32 = (l + r) / 2.0;
            if num >= mid {
                res |= tmp;
                l = mid;
            } else {
                r = mid;
            }
            tmp >>= 1;
        }
        return res;
    }

    /*
     * merge lon/lat to 64 bits. (alternately)
     */
    fn merge_bis(lon: u32, lat: u32) -> u64 {
        let to_u64 = |n: u32| -> u64 {
            let mut n = n as u64;
            n = (n | (n << 16)) & 0x0000ffff0000ffff;
            n = (n | (n << 8)) & 0x00ff00ff00ff00ff;
            n = (n | (n << 4)) & 0x0f0f0f0f0f0f0f0f;
            n = (n | (n << 2)) & 0x3333333333333333;
            n = (n | (n << 1)) & 0x5555555555555555;
            return n;
        };
        return (to_u64(lon) << 1) | to_u64(lat);
    }

    const CHARS: [char; 32] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j',
        'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    /*
     * convert merged lon/lat to base32
     */
    fn base32(mut bits: u64, level: GeoLevel) -> String {
        let mut hash = String::new();
        for _ in 0..level.get_level() {
            let tmp = bits & 0xf800000000000000;
            hash.push(Self::CHARS[(tmp >> 59) as usize]);
            bits <<= 5;
        }
        return hash;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level10);
        println!("hash10: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level9);
        println!("hash9: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level8);
        println!("hash8: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level7);
        println!("hash7: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level6);
        println!("hash6: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level5);
        println!("hash5: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level4);
        println!("hash4: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level3);
        println!("hash3: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level2);
        println!("hash2: {}", hash);
        let hash = GeoHash::get_geo_hash(121.506379, 31.245414, GeoLevel::Level1);
        println!("hash1: {}", hash);
    }
}
