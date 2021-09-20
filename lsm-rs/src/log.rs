use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader},
};

use chrono::Utc;
use rb_tree::RBMap;

use crate::{reader::Reader, writer::Writer};

pub struct Log {
    cache_file_path: String,
    cache_file: File,
}

impl Log {
    pub fn new(base_path: &str) -> Log {
        let log_base_path = format!("{}/log", base_path);
        fs::create_dir_all(&log_base_path).unwrap();
        let cache_file_path = format!("{}/cache.log", &log_base_path);
        let cache_file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&cache_file_path)
            .unwrap();
        return Log {
            cache_file_path,
            cache_file,
        };
    }

    pub fn append(&mut self, key: &String, value: Option<&String>) -> io::Result<()> {
        return Writer::write_by_seek(&mut self.cache_file, key, value);
    }

    pub fn build_map(path: &String) -> io::Result<RBMap<String, Option<String>>> {
        let mut reader: BufReader<File> = BufReader::new(OpenOptions::new().read(true).open(path)?);
        let mut map: RBMap<String, Option<String>> = RBMap::new();
        while let Ok(Some((k, v))) = Reader::read_by_seek(&mut reader) {
            map.insert(k, v);
        }
        return Ok(map);
    }

    pub fn save_cache_file(&mut self) -> io::Result<String> {
        let saved_log_path: String = format!("{}.log", Utc::now().timestamp_nanos());
        fs::rename(&self.cache_file_path, &saved_log_path)?;
        self.cache_file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.cache_file_path)?;
        return Ok(saved_log_path);
    }
}
