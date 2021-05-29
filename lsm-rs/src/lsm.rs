use rb_tree::RBMap;
use std::{
    mem::{self, size_of_val},
    sync::{Arc, RwLock},
    thread,
};

use crate::store::Store;

pub struct Lsm {
    cache_size: usize,
    mem_table: RBMap<String, Option<String>>,
    immutable_table: Option<Arc<RwLock<RBMap<String, Option<String>>>>>,
    store: Arc<RwLock<Store>>,
}

impl Lsm {
    pub fn init(path: &str, cache_size: usize, db_level: usize) -> Self {
        let store = Store::init(path, db_level);
        let mem_table = store.get_mem_table();
        return Lsm {
            cache_size,
            mem_table,
            immutable_table: None,
            store: Arc::new(RwLock::new(store)),
        };
    }

    pub fn insert(&mut self, key: &String, val: &String) {
        if let Ok(store) = self.store.write() {
            //FIXME 写日志成功但是写缓存失败，如果确保在此刻重启后日志里的数据无效？(无解)
            store.log(key, Some(val)).unwrap();
            self.mem_table.insert(key.to_owned(), Some(val.to_owned()));
            if !self.mem_table.is_empty() && size_of_val(&self.mem_table) >= self.cache_size {
                store.replace_log();
                self.immutable_table = Some(Arc::new(RwLock::new(mem::replace(
                    &mut self.mem_table,
                    RBMap::new(),
                ))));
            }
            if self.immutable_table.is_some() {
                let store = self.store.clone();
                let immutable_table = self.immutable_table.as_ref().unwrap().clone();
                thread::spawn(move || store.write().unwrap().store(immutable_table));
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<String> {
        if self.mem_table.contains_key(key) {
            return self.mem_table.get(key).unwrap().clone();
        }
        if let Some(immutable_table) = self.immutable_table.as_ref() {
            if let Ok(table) = immutable_table.read() {
                return table.get(key).unwrap().clone();
            }
        }
        return self.store.read().unwrap().search_by_key(key);
    }

    pub fn remove(&mut self, key: &String) {
        self.store.write().unwrap().log(key, None).unwrap();
        self.mem_table.insert(key.to_owned(), None);
    }
}
