use rb_tree::RBMap;
use std::{
    mem::{self, size_of_val},
    sync::{Arc, RwLock},
    thread,
};

use crate::store::Store;

pub struct Lsm {
    mem_table: RBMap<String, Option<String>>,
    cache_size: usize,
    store: Arc<RwLock<Store>>,
}

impl Lsm {
    pub fn init(cache_size: usize, path: &str) -> Self {
        let store = Store::init(path);
        let mem_table = store.get_mem_table();
        return Lsm {
            mem_table,
            cache_size,
            store: Arc::new(RwLock::new(store)),
        };
    }

    pub fn insert(&mut self, key: &String, val: &String) {
        if let Ok(store) = self.store.write() {
            store.log(key, Some(val)).unwrap();
            self.mem_table.insert(key.to_owned(), Some(val.to_owned()));
            if size_of_val(&self.mem_table) >= self.cache_size {
                store.replace_log();
                let immutable_mem_table = mem::replace(&mut self.mem_table, RBMap::new());
                let store = self.store.clone();
                thread::spawn(move || store.write().unwrap().store(immutable_mem_table));
            }
        }
    }

    pub fn get(&mut self, key: &String) -> Option<String> {
        if self.mem_table.contains_key(key) {
            return self.mem_table.get(key).unwrap().clone();
        } else {
            return self.store.read().unwrap().search_by_key(key);
        }
    }

    pub fn remove(&mut self, key: &String) {
        self.store.write().unwrap().log(key, None).unwrap();
        self.mem_table.insert(key.to_owned(), None);
    }
}
