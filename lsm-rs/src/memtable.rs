use std::{
    mem,
    sync::{Arc, RwLock},
};

use rb_tree::RBMap;

pub struct MemTable {
    pub table: RBMap<String, Option<String>>,
    pub immut_tables: Arc<RwLock<Vec<(String, RBMap<String, Option<String>>)>>>,
    pub capicaty: usize,
}

impl MemTable {
    pub fn new(capicaty: usize) -> Self {
        return MemTable {
            table: RBMap::new(),
            immut_tables: Arc::new(RwLock::new(Vec::new())), // 必须为有序结构，保证查询时的最新值
            capicaty,
        };
    }

    pub fn insert(&mut self, key: String, value: Option<String>) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &String) -> (bool, Option<String>) {
        if self.table.contains_key(key) {
            return (
                true,
                match self.table.get(key) {
                    Some(v) => v.clone(),
                    _ => None,
                },
            );
        }
        if let Ok(tables) = self.immut_tables.read() {
            // 反向读取immut_tabls，反向为最新值
            for table in tables.iter().rev().map(|p| &p.1) {
                if table.contains_key(key) {
                    return (
                        true,
                        match table.get(key) {
                            Some(v) => v.clone(),
                            _ => None,
                        },
                    );
                }
            }
        }
        return (false, None);
    }

    pub fn save_table(&mut self, saved_log_path: &String) {
        self.immut_tables.write().unwrap().push((
            saved_log_path.to_string(),
            mem::replace(&mut self.table, RBMap::new()),
        ));
    }
}
