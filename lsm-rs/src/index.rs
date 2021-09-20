use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Position {
    pub level: usize,
    pub path: String,
    pub start_key: String,
    pub end_key: String,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            self.start_key.cmp(&other.start_key),
            self.end_key.cmp(&other.end_key),
        ) {
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl Position {
    fn new(level: usize, path: String, start_key: String, end_key: String) -> Self {
        Self {
            level,
            path,
            start_key,
            end_key,
        }
    }
}

#[derive(Debug)]
pub struct Index {
    level: usize,
    file: File,
    path_indexes: HashMap<String, (Arc<Position>, bool)>,
    key_indexes: Vec<BTreeSet<Arc<Position>>>,
}

impl Index {
    pub fn new(base_path: &String, level: usize) -> Self {
        let index_dir = format!("{}/index", base_path);
        fs::create_dir_all(&index_dir).unwrap();
        let mut index = Self {
            level,
            file: OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(format!("{}/{}.index", index_dir, "sstable"))
                .unwrap(),
            path_indexes: HashMap::new(),
            key_indexes: vec![BTreeSet::new(); level],
        };
        index.init().unwrap();
        return index;
    }

    fn init(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut reader: BufReader<&File> = BufReader::new(&self.file);
        let mut buf = String::new();
        while let Ok(size) = reader.read_line(&mut buf) {
            if size == 0 {
                break;
            } else {
                let columns: Vec<&str> = buf.split_whitespace().collect();
                let level: usize = columns[0].parse::<usize>().unwrap();
                let path: &str = columns[1];
                let start_key: String = columns[2].to_string();
                let end_key: String = columns[3].to_string();
                if columns.len() == 4 {
                    let position = Arc::new(Position {
                        level,
                        path: path.to_string(),
                        start_key,
                        end_key,
                    });
                    self.key_indexes[level].insert(position.clone());
                    self.path_indexes
                        .insert(path.to_string(), (position.clone(), false));
                }
                buf.clear();
            }
        }
        self.file.seek(SeekFrom::End(0))?;
        return Ok(());
    }

    pub fn add(
        &mut self,
        current_level: usize,
        path: &String,
        start_key: &String,
        end_key: &String,
    ) -> io::Result<()> {
        self.file.write_all(
            format!("{} {} {} {}\n", current_level, path, start_key, end_key).as_bytes(),
        )?;
        let position = Arc::new(Position::new(
            current_level,
            path.to_string(),
            start_key.to_string(),
            end_key.to_string(),
        ));
        self.key_indexes[current_level].insert(position.clone());
        self.path_indexes
            .insert(position.path.clone(), (position, false));
        return Ok(());
    }

    pub fn clear(&mut self, useless_positions: &Vec<Arc<Position>>) -> io::Result<()> {
        for position in useless_positions {
            if let Some(p) = self.path_indexes.remove(&position.path) {
                self.key_indexes[p.0.level].remove(&p.0);
            }
        }
        return Ok(());
    }

    pub fn get_random_position_in_level(&mut self, level: usize) -> Option<Arc<Position>> {
        for position in self.key_indexes[level].iter() {
            if let Some(p) = self.path_indexes.get_mut(&position.path) {
                if !p.1 {
                    p.1 = true;
                    return Some(p.0.clone());
                }
            }
        }
        return None;
    }

    pub fn get_hit_positions_in_level(
        &mut self,
        level: usize,
        position: Arc<Position>,
    ) -> Vec<Arc<Position>> {
        let mut res = vec![];
        let positions = self.key_indexes[level].range(position.clone()..=position);
        for kp in positions {
            if let Some(pp) = self.path_indexes.get_mut(&kp.path) {
                if !pp.1 {
                    pp.1 = true;
                    res.push(pp.0.clone());
                }
            }
        }
        return res;
    }

    /*
     * level 0：可能会出现区间重复的sstable
     * level 1..n：start_key == end_key时，只会命中一个sstable（合并方式保证每一层sstable文件没有交集）
     */
    pub fn get_hit_path_in_db(&self, start_key: &String, end_key: &String) -> Vec<Arc<Position>> {
        let position = Arc::new(Position::new(
            0,
            start_key.to_string(),
            end_key.to_string(),
            String::default(),
        ));
        let mut res = vec![];
        for i in 0..self.level {
            let mut positions: Vec<Arc<Position>> = self.key_indexes[i]
                .range(position.clone()..=position.clone())
                .map(|p| p.clone())
                .collect();
            if !positions.is_empty() {
                res.append(&mut positions);
            }
        }
        return res;
    }
}
