use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::Path,
    sync::{Arc, RwLock},
};

use chrono::Utc;
use rb_tree::RBMap;

use crate::{
    index::{Index, Position},
    reader::Reader,
    writer::Writer,
};

pub struct SSTable {
    pub path: String,
    pub level: usize,
    pub level_capacity: usize,
    pub index: Arc<RwLock<Index>>,
}

impl SSTable {
    pub fn new(base_path: &str, level: usize, level_capacity: usize) -> Self {
        let path = format!("{}/sstable", base_path);
        fs::create_dir_all(&path).unwrap();
        for i in 0..level {
            fs::create_dir_all(format!("{}/{}", &path, i)).unwrap()
        }
        let index = Arc::new(RwLock::new(Index::new(&path, level)));
        return Self {
            path,
            level,
            level_capacity,
            index,
        };
    }

    pub fn get(&self, key: &String) -> io::Result<Option<String>> {
        let positions = self.index.read().unwrap().get_hit_path_in_db(key, key);
        for path in positions.iter().map(|p| &p.path) {
            let val = Reader::search_by_key(&Path::new(&path), key);
            if val.is_ok() && val.as_ref().unwrap().is_some() {
                return val;
            }
        }
        return Ok(None);
    }

    pub fn save(
        sstable_path: String,
        level: usize,
        level_capacity: usize,
        saved_log_path: String,
        immut_tables: Arc<RwLock<Vec<(String, RBMap<String, Option<String>>)>>>,
        index: Arc<RwLock<Index>>,
    ) -> io::Result<()> {
        // minor compaction（持久化immut_tables）
        let file_path = format!("{}/0/{}.sst", sstable_path, Utc::now().timestamp_nanos());
        let file = &mut OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)?;
        let mut start_key: Option<String> = None;
        let mut end_key: Option<String> = None;
        let mut start_tmp: Option<&String> = None;
        let mut end_tmp: Option<&String> = None;
        if let Some(table) = immut_tables
            .read()
            .unwrap()
            .iter()
            .filter(|pair| pair.0 == saved_log_path)
            .map(|pair| &pair.1)
            .next()
        {
            for (k, v) in table.iter() {
                Writer::write_by_seek(file, k, v.as_ref())?;
                if start_key.is_none() {
                    start_tmp.replace(k);
                }
                end_tmp.replace(k);
            }
            start_key.replace(start_tmp.unwrap().clone());
            end_key.replace(end_tmp.unwrap().clone());
        }
        if let Ok(mut tables) = immut_tables.write() {
            let index = tables
                .iter()
                .position(|pair| pair.0 == saved_log_path)
                .unwrap();
            tables.remove(index);
        }
        fs::remove_file(saved_log_path)?;
        if start_key.is_some() && end_key.is_some() {
            index
                .write()
                .unwrap()
                .add(0, &file_path, &start_key.unwrap(), &end_key.unwrap())?;
        }

        // major compcation（校验每层文件并合并）
        return Self::compaction(&sstable_path, &level, &level_capacity, index);
    }

    fn compaction(
        sstable_path: &String,
        level: &usize,
        level_capacity: &usize,
        index: Arc<RwLock<Index>>,
    ) -> io::Result<()> {
        for i in 0..*level {
            if fs::read_dir(format!("{}/{}", sstable_path, i))?.count() > *level_capacity {
                let mut write_index = index.write().unwrap();
                if let Some(position) = write_index.get_random_position_in_level(i) {
                    let merge_level = i + 1;
                    let mut positions: Vec<Arc<Position>> =
                        write_index.get_hit_positions_in_level(merge_level, position.clone());
                    drop(write_index);
                    positions.insert(0, position);
                    Self::merge(sstable_path, merge_level, positions, index.clone())?;
                }
            } else if i == 0 {
                break;
            }
        }
        return Ok(());
    }

    fn merge(
        path: &String,
        merge_level: usize,
        positions: Vec<Arc<Position>>,
        index: Arc<RwLock<Index>>,
    ) -> io::Result<()> {
        if positions.len() == 1 {
            let new_path = format!(
                "{}/{}/{}.sst",
                path,
                merge_level,
                Utc::now().timestamp_nanos()
            );
            let position = &positions[0];
            if let Ok(mut locked_index) = index.write() {
                // 单文件合并直接修改文件名
                fs::rename(&position.path, &new_path)?;
                locked_index.add(
                    merge_level,
                    &new_path,
                    &position.start_key,
                    &position.end_key,
                )?;
                locked_index.clear(&positions)?;
            }
        } else {
            let mut readers: Vec<BufReader<File>> = positions
                .iter()
                .map(|p| BufReader::new(OpenOptions::new().read(true).open(&p.path).unwrap()))
                .collect();
            let tmp_file_path = format!(
                "{}/{}/{}.tmp",
                path,
                merge_level,
                Utc::now().timestamp_nanos()
            );
            let mut writer = BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&tmp_file_path)?,
            );

            let mut start_key: Option<String> = None;
            let mut end_key: Option<String> = None;

            // 归并所有文件（文件已按层级从小到大排列）
            let mut heap: BinaryHeap<(Reverse<String>, Reverse<usize>, Option<String>)> =
                BinaryHeap::new();
            for (i, reader) in readers.iter_mut().enumerate() {
                if let Some((k, v)) = Reader::read_by_seek(reader)? {
                    heap.push((Reverse(k), Reverse(i), v));
                }
            }
            // 每个文件取一条数据，保证每个文件的第一条数据以key和按照文件层级顺序排列
            let mut pre: Option<(Reverse<String>, Reverse<usize>, Option<String>)> = None;
            while !heap.is_empty() {
                let tmp = heap.pop().unwrap();
                if let Some((k, v)) = Reader::read_by_seek(&mut readers[tmp.1 .0])? {
                    heap.push((Reverse(k), tmp.1, v));
                }
                if pre.is_none() || (pre.is_some() && pre.as_ref().unwrap().0 != tmp.0) {
                    if start_key.is_none() {
                        start_key.replace((tmp.0).0.clone());
                    }
                    end_key.replace((tmp.0).0.clone());
                    Writer::write_by_seek(&mut writer, &((tmp.0).0), tmp.2.as_ref())?;
                    pre = Some(tmp);
                }
            }

            let new_file_path = tmp_file_path.replace(".tmp", ".sst");
            fs::rename(&tmp_file_path, &new_file_path)?;

            if let Ok(mut locked_index) = index.write() {
                locked_index.add(
                    merge_level,
                    &new_file_path,
                    &start_key.unwrap(),
                    &end_key.unwrap(),
                )?;
                locked_index.clear(&positions)?;
            }
            for position in positions.iter() {
                fs::remove_file(&position.path)?;
            }
        }
        return Ok(());
    }
}
