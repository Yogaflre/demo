use crate::{common::error::Error, encoding::sds::Sds};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

const ACTIVE_INDEX: usize = 0;
const PASSIVE_INDEX: usize = 1;

enum RehashType {
    Expand,
    Shrink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dict<V>
where
    V: Clone,
{
    load_factor: f32,
    maps: [HashMap<Arc<Sds>, V>; 2],
    rehashing: i8, // rehashing进度
}

impl<V> Dict<V>
where
    V: Clone,
{
    pub fn new(load_factor: f32) -> Self {
        Self {
            load_factor,
            maps: [HashMap::with_capacity(4), HashMap::default()],
            rehashing: -1,
        }
    }

    pub fn dict_add(&mut self, key: Arc<Sds>, val: V) -> Result<bool, Error> {
        if self.is_rehashing() {
            self.rehash_step();
        } else if let Some(reahsh_type) = self.need_rehash() {
            self.resize_dict(reahsh_type)?;
        }

        if self.exist(key.clone()) {
            return Ok(false);
        }

        self.add_entry(key, val);
        return Ok(true);
    }

    pub fn dict_replace(&mut self, key: Arc<Sds>, val: V) -> Result<bool, Error> {
        if self.is_rehashing() {
            self.rehash_step();
        }

        if self.exist(key.clone()) {
            return Ok(false);
        }

        self.add_entry(key, val);
        return Ok(true);
    }

    /*
     * 因为渐进式rehash，所以需要使用可变引用
     */
    pub fn dict_get(&mut self, key: Arc<Sds>) -> Result<Option<V>, Error> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        }

        let mut obj = self.get_value(key.clone(), ACTIVE_INDEX);
        if obj.is_none() && is_rehashing {
            obj = self.get_value(key, PASSIVE_INDEX);
        }

        return Ok(obj);
    }

    pub fn dict_contanins_key(&mut self, key: Arc<Sds>) -> Result<bool, Error> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        }

        let mut exists = self.maps[ACTIVE_INDEX].contains_key(&key);
        if !exists && is_rehashing {
            exists = self.maps[PASSIVE_INDEX].contains_key(&key);
        }

        return Ok(exists);
    }

    //TODO 随机获取一个key
    pub fn dict_get_random_key(&mut self) -> Result<Option<(Sds, V)>, Error> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        }

        return Ok(None);
    }

    pub fn dict_delete(&mut self, key: Arc<Sds>) -> Result<Option<(Arc<Sds>, V)>, Error> {
        let is_rehashing = self.is_rehashing();
        if is_rehashing {
            self.rehash_step();
        } else if let Some(reahsh_type) = self.need_rehash() {
            self.resize_dict(reahsh_type)?;
        }

        let mut entry = self.remove_entry(key.clone(), ACTIVE_INDEX);
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

    pub fn dict_release(&mut self) -> Result<bool, Error> {
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

    fn resize_dict(&mut self, rehash_type: RehashType) -> Result<(), Error> {
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

    fn add_entry(&mut self, key: Arc<Sds>, val: V) {
        self.maps[ACTIVE_INDEX].insert(key, val);
    }

    fn get_value(&self, key: Arc<Sds>, index: usize) -> Option<V> {
        return self.maps[index].get(&key).map(|v| v.clone());
    }

    fn remove_entry(&mut self, key: Arc<Sds>, index: usize) -> Option<(Arc<Sds>, V)> {
        return self.maps[index].remove_entry(&key);
    }

    fn exist(&self, key: Arc<Sds>) -> bool {
        return self.maps[ACTIVE_INDEX].contains_key(&key)
            || (self.is_rehashing() && self.maps[PASSIVE_INDEX].contains_key(&key));
    }
}

#[test]
fn test_dict() {
    use rand::Rng;
    use std::time::Instant;

    let mut dict: Dict<Sds> = Dict::new(0.8);
    let count: u32 = 5000;

    // add
    let time = Instant::now();
    for i in 0..count {
        dict.dict_add(Arc::new(Sds::new(&i.to_le_bytes())), Sds::new(b"value"))
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
        let key = Arc::new(Sds::new(&i.to_le_bytes()));
        assert!(dict.dict_get(key).unwrap().is_some());
    }
    println!("get {} time: {:?}", count, time.elapsed());

    // random get
    let mut rng = rand::thread_rng();
    let time = Instant::now();
    for _ in 0..count {
        let key = Arc::new(Sds::new(&(rng.gen::<u32>() % count).to_le_bytes()));
        assert!(dict.dict_get(key).unwrap().is_some());
    }
    println!("random get {} time: {:?}", count, time.elapsed());

    // missing get
    let time = Instant::now();
    for _ in 0..count {
        let key = Arc::new(Sds::new(&"X".as_bytes()));
        assert!(dict.dict_get(key).unwrap().is_none());
    }
    println!("missing get {} time: {:?}", count, time.elapsed());

    // remove
    let time = Instant::now();
    for i in 0..count {
        let key = Arc::new(Sds::new(&i.to_le_bytes()));
        assert!(dict.dict_delete(key).unwrap().is_some());
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
