use std::{io, mem::size_of_val, thread};

use crate::{log::Log, memtable::MemTable, sstable::SSTable};

pub struct Lsm {
    mem_table: MemTable,
    log: Log,
    sstable: SSTable,
}

impl Lsm {
    pub fn new(path: &str, mem_table_capicaty: usize, level: usize, level_capicatiy: usize) -> Lsm {
        return Lsm {
            mem_table: MemTable::new(mem_table_capicaty),
            log: Log::new(path),
            sstable: SSTable::new(path, level, level_capicatiy),
        };
    }

    pub fn insert(&mut self, key: &String, val: &String) -> io::Result<()> {
        //FIXME 写日志成功但是写缓存失败，如果确保在此刻重启后日志里的数据无效？(无解)
        self.log.append(key, Some(val))?;
        self.mem_table
            .insert(key.to_string(), Some(val.to_string()));
        return self.check_capacity();
    }

    pub fn get(&self, key: &String) -> io::Result<Option<String>> {
        let val: (bool, Option<String>) = self.mem_table.get(key);
        if !val.0 {
            return self.sstable.get(key);
        } else {
            return Ok(val.1);
        }
    }

    pub fn remove(&mut self, key: &String) -> io::Result<()> {
        self.log.append(key, None)?;
        self.mem_table.insert(key.to_string(), None);
        return self.check_capacity();
    }

    fn check_capacity(&mut self) -> io::Result<()> {
        if size_of_val(&self.mem_table.table) > self.mem_table.capicaty {
            let saved_log_path: String = self.log.save_cache_file()?;
            self.mem_table.save_table(&saved_log_path);
            let sstable_path = self.sstable.path.clone();
            let sstable_level = self.sstable.level.clone();
            let sstable_level_capacity = self.sstable.level_capacity.clone();
            let index = self.sstable.index.clone();
            let table = self.mem_table.immut_tables.clone();
            thread::spawn(move || {
                SSTable::save(
                    sstable_path,
                    sstable_level,
                    sstable_level_capacity,
                    saved_log_path,
                    table,
                    index,
                )
                .unwrap();
            });
        }
        return Ok(());
    }
}
