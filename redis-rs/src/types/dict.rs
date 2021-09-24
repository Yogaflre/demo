use std::collections::HashMap;

use crate::encoding::sds::Sds;

use super::object::Object;

const ACTIVE_INDEX: usize = 0;
const PASSIVE_INDEX: usize = 1;

enum RehashType {
    Expand,
    Shrink,
}

pub struct Dict {
    load_factor: f32,
    maps: [HashMap<Sds, Box<dyn Object>>; 2],
    rehashing: i8, // rehashing进度
}

impl Dict {
    pub fn new(load_factor: f32) -> Self {
        Self {
            load_factor,
            maps: [HashMap::with_capacity(4), HashMap::default()],
            rehashing: -1,
        }
    }

    pub fn dict_add(&mut self, key: Sds, val: Box<dyn Object>) -> Result<bool, String> {
        if self.is_rehashing() {
            self.rehash_step();
        } else if let Some(reahsh_type) = self.need_rehash() {
            self.resize_dict(reahsh_type)?;
        }

        if self.exist(&key) {
            return Ok(false);
        }

        self.add_entry(key, val);
        return Ok(true);
    }

    pub fn dict_replace(&mut self, key: Sds, val: Box<dyn Object>) -> Result<bool, String> {
        if self.is_rehashing() {
            self.rehash_step();
        }

        if self.exist(&key) {
            return Ok(false);
        }

        self.add_entry(key, val);
        return Ok(true);
    }

    /*
     * 因为渐进式rehash，所以需要使用可变引用
     */
    pub fn dict_fetch_value(&mut self, key: &Sds) -> Result<Option<Box<dyn Object>>, String> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        }

        let mut obj = self.get_value(key, ACTIVE_INDEX);
        if obj.is_none() && is_rehashing {
            obj = self.get_value(key, PASSIVE_INDEX);
        }

        return Ok(obj);
    }

    //TODO 随机获取一个key
    pub fn dict_get_random_key(&mut self) -> Result<Option<(Sds, Box<dyn Object>)>, String> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        }

        return Ok(None);
    }

    pub fn dict_delete(&mut self, key: &Sds) -> Result<Option<(Sds, Box<dyn Object>)>, String> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        } else if let Some(reahsh_type) = self.need_rehash() {
            self.resize_dict(reahsh_type)?;
        }

        let mut entry = self.remove_entry(key, ACTIVE_INDEX);
        if entry.is_none() && is_rehashing {
            entry = self.remove_entry(key, PASSIVE_INDEX);
        }

        return Ok(entry);
    }

    pub fn dict_size(&self) -> usize {
        let mut size = self.maps[ACTIVE_INDEX].len();
        if self.is_rehashing() {
            size += self.maps[PASSIVE_INDEX].len();
        }
        return size;
    }

    pub fn dict_release(&mut self) -> Result<bool, String> {
        drop(self);
        return Ok(true);
    }

    fn is_rehashing(&self) -> bool {
        return self.rehashing != -1;
    }

    fn rehash_step(&mut self) {
        // 借用规则必须得clone一份key后再删除
        let remove_key = self.maps[PASSIVE_INDEX]
            .keys()
            .into_iter()
            .next()
            .map(|k| k.clone());
        if let Some(key) = remove_key {
            let (k, v) = self.maps[PASSIVE_INDEX].remove_entry(&key).unwrap();
            self.add_entry(k, v);
        } else {
            // 结束rehash
            self.maps[PASSIVE_INDEX].shrink_to_fit();
            self.rehashing = -1;
        }
    }

    fn need_rehash(&self) -> Option<RehashType> {
        let factor =
            self.maps[ACTIVE_INDEX].len() as f32 / (self.maps[ACTIVE_INDEX].capacity()) as f32;
        if factor > self.load_factor {
            return Some(RehashType::Expand);
        } else if factor < 0.1 {
            return Some(RehashType::Shrink);
        }
        return None;
    }

    fn resize_dict(&mut self, rehash_type: RehashType) -> Result<(), String> {
        match rehash_type {
            RehashType::Expand => {
                self.maps[PASSIVE_INDEX] =
                    HashMap::with_capacity(self.maps[ACTIVE_INDEX].capacity() * 2);
                self.maps.swap(ACTIVE_INDEX, PASSIVE_INDEX);
                self.rehashing = 0;
            }
            // FIXME 缩容没有使用渐进式rehash（需要考虑如何与扩容时的渐进式rehash冲突）
            RehashType::Shrink => {
                self.maps[ACTIVE_INDEX].shrink_to_fit();
            }
        };
        return Ok(());
    }

    fn add_entry(&mut self, key: Sds, val: Box<dyn Object>) {
        self.maps[ACTIVE_INDEX].insert(key, val);
    }

    fn get_value(&self, key: &Sds, index: usize) -> Option<Box<dyn Object>> {
        return self.maps[index].get(key).map(|o| o.clone());
    }

    fn remove_entry(&mut self, key: &Sds, index: usize) -> Option<(Sds, Box<dyn Object>)> {
        return self.maps[index].remove_entry(key);
    }

    fn exist(&self, key: &Sds) -> bool {
        return self.maps[ACTIVE_INDEX].contains_key(key)
            || (self.is_rehashing() && self.maps[PASSIVE_INDEX].contains_key(&key));
    }
}

#[test]
fn test_dict() {
    use crate::types::strings::StringObject;
    use rand::Rng;
    use std::time::Instant;

    let mut dict: Dict = Dict::new(0.8);
    let count: u32 = 5000;

    // add
    let time = Instant::now();
    for i in 0..count {
        dict.dict_add(
            Sds::new(&i.to_string().chars().collect::<Vec<char>>()),
            Box::new(StringObject::default()),
        )
        .unwrap();
    }
    println!("add {} time: {:?}", count, time.elapsed());

    // size
    assert_eq!(count as usize, dict.dict_size());

    while dict.is_rehashing() {
        dict.rehash_step();
    }

    // get
    let time = Instant::now();
    for i in 0..count {
        let key = Sds::new(&i.to_string().chars().collect::<Vec<char>>());
        assert!(dict.dict_fetch_value(&key).unwrap().is_some());
    }
    println!("get {} time: {:?}", count, time.elapsed());

    // random get
    let mut rng = rand::thread_rng();
    let time = Instant::now();
    for _ in 0..count {
        let key = Sds::new(
            &(rng.gen::<u32>() % count)
                .to_string()
                .chars()
                .collect::<Vec<char>>(),
        );
        assert!(dict.dict_fetch_value(&key).unwrap().is_some());
    }
    println!("random get {} time: {:?}", count, time.elapsed());

    // missing get
    let time = Instant::now();
    for _ in 0..count {
        let key = Sds::new(&['X']);
        assert!(dict.dict_fetch_value(&key).unwrap().is_none());
    }
    println!("missing get {} time: {:?}", count, time.elapsed());

    // remove
    let time = Instant::now();
    for i in 0..count {
        let key = Sds::new(&i.to_string().chars().collect::<Vec<char>>());
        assert!(dict.dict_delete(&key).unwrap().is_some());
    }
    println!("remove {} time: {:?}", count, time.elapsed());

    // capacity
    assert_eq!(0, dict.dict_size());
    println!(
        "map0 capacity: {} | map1 capacity: {}",
        dict.maps[0].capacity(),
        dict.maps[1].capacity()
    );

    dict.dict_release().unwrap();
}
