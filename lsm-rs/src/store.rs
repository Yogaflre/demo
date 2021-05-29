use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
    mem::replace,
    panic,
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use rb_tree::RBMap;

#[derive(Debug)]
pub struct Store {
    log: String,
    db: String,
    db_index: usize, //0 level的文件个数(mem_table只会持久化到0 level)
    db_level: usize, //db层级
    index: String,
}
impl Store {
    pub fn init(path: &str, db_level: usize) -> Self {
        let log = format!("{}/log", path);
        let db = format!("{}/db", path);
        let index = format!("{}/index", path);
        fs::create_dir_all(&log).unwrap();
        fs::create_dir_all(&db).unwrap();
        for i in 0..db_level {
            fs::create_dir_all(format!("{}/{}", db, i)).unwrap()
        }
        fs::create_dir_all(&index).unwrap();

        let db_file_count = fs::read_dir(format!("{}/0", db)).unwrap().count();
        let mut store = Store {
            log,
            db,
            db_index: db_file_count,
            db_level,
            index,
        };

        //每次启动清理残留的save.log
        store
            .store(Arc::new(RwLock::new(store.get_immutable_mem_table())))
            .unwrap();

        //启动merge定时任务
        let db_path = store.db.clone();
        thread::spawn(move || loop {
            Store::merge(&db_path, db_level).unwrap();
            thread::sleep(Duration::from_secs(60));
        });
        return store;
    }

    pub fn replace_log(&self) {
        fs::rename(
            format!("{}/cache.log", self.log),
            format!("{}/save.log", self.log),
        )
        .unwrap();
    }

    pub fn log(&self, key: &String, val: Option<&String>) -> io::Result<()> {
        let mut log_file = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("{}/cache.log", self.log))?,
        );
        return Self::write_kv(&mut log_file, key, val);
    }

    pub fn store(
        &mut self,
        immutable_table: Arc<RwLock<RBMap<String, Option<String>>>>,
    ) -> io::Result<()> {
        if !immutable_table.read().unwrap().is_empty() {
            let mut db_file = BufWriter::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(format!("{}/0/{}.db", self.db, self.db_index))?,
            );
            for (k, v) in immutable_table.read().unwrap().iter() {
                Self::write_kv(&mut db_file, &k, v.as_ref())?;
            }
            self.db_index += 1;
            replace(&mut immutable_table.write().ok(), None);
            fs::remove_file(format!("{}/save.log", self.log))?;
        }
        return Ok(());
    }

    pub fn search_by_key(&self, key: &String) -> Option<String> {
        for i in (0..self.db_level).rev() {
            let mut level_paths: Vec<PathBuf> = fs::read_dir(format!("{}/{}", self.db, i))
                .unwrap()
                .filter_map(Result::ok)
                .map(|dir| dir.path())
                .collect();
            level_paths.sort_by(|a, b| b.as_path().cmp(a.as_path()));
            for path in level_paths {
                let mut reader = BufReader::new(File::open(path).unwrap());
                loop {
                    if let Some((k, v)) = Self::get_kv(&mut reader) {
                        if *key == k {
                            return v;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        return None;
    }

    pub fn get_mem_table(&self) -> RBMap<String, Option<String>> {
        return Self::get_table(format!("{}/cache.log", self.log));
    }

    pub fn get_immutable_mem_table(&self) -> RBMap<String, Option<String>> {
        return Self::get_table(format!("{}/save.log", self.log));
    }

    fn get_table(path: String) -> RBMap<String, Option<String>> {
        let mut map: RBMap<String, Option<String>> = RBMap::new();
        let mut reader = BufReader::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(path)
                .unwrap(),
        );
        loop {
            if let Some((key, val)) = Self::get_kv(&mut reader) {
                map.insert(key, val);
            } else {
                break;
            }
        }
        return map;
    }

    pub fn write_kv(
        writer: &mut BufWriter<File>,
        key: &String,
        val: Option<&String>,
    ) -> io::Result<()> {
        let key_bytes = key.as_bytes();
        if key_bytes.len() > 255 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("key size invalid! key_size:{}", key_bytes.len(),),
            ));
        }
        if let Some(v) = val {
            let val_bytes = v.as_bytes();
            if val_bytes.len() > 255 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("value size invalid! value_bytes:{}", val_bytes.len()),
                ));
            }
            writer.write_all(&(false as u8).to_le_bytes())?;
            writer.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(&(val_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(key_bytes)?;
            writer.write_all(val_bytes)?;
        } else {
            writer.write_all(&(true as u8).to_le_bytes())?;
            writer.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(key_bytes)?;
        }
        return Ok(());
    }

    pub fn get_kv(reader: &mut BufReader<File>) -> Option<(String, Option<String>)> {
        let mut buf: Vec<u8> = vec![0; 1];
        match reader.read_exact(&mut buf) {
            Ok(_) => {
                let is_delete = buf[0] == 1_u8;
                if is_delete {
                    buf = vec![0; 1];
                    reader.read_exact(&mut buf).unwrap();
                    let key_size = buf[0];

                    buf = vec![0; key_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let key = String::from_utf8(buf.clone()).unwrap();

                    return Some((key, None));
                } else {
                    buf = vec![0; 2];
                    reader.read_exact(&mut buf).unwrap();
                    let key_size = buf[0];
                    let value_size = buf[1];

                    buf = vec![0; key_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let key = String::from_utf8(buf.clone()).unwrap();

                    buf = vec![0; value_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let val = String::from_utf8(buf.clone()).unwrap();

                    return Some((key, Some(val)));
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::UnexpectedEof => return None,
                _ => panic!("read log error: {}", error),
            },
        }
    }

    //归并每层达到阈值的文件个数
    fn merge(db_path: &str, db_level: usize) -> io::Result<()> {
        //FIXME 因为mem会不断的持久化到0层，所以在0层归并完有可能出现文件序号错乱的问题
        for i in 0..(db_level - 1) {
            let low_paths: Vec<PathBuf> = fs::read_dir(format!("{}/{}", db_path, i))?
                .filter_map(Result::ok)
                .map(|dir| dir.path())
                .collect();
            //大于阈值才归并
            if low_paths.len() >= 3 {
                let mut low_readers: Vec<BufReader<File>> = low_paths
                    .iter()
                    .filter_map(|path| OpenOptions::new().read(true).open(path).ok())
                    .map(|file| BufReader::new(file))
                    .collect();
                let file_name = format!(
                    "{}/{}/{}.unfinished",
                    db_path,
                    i + 1,
                    fs::read_dir(format!("{}/{}", db_path, i + 1))?
                        .filter_map(Result::ok)
                        .filter(|dir| dir.path().ends_with(".db"))
                        .count()
                );
                let mut high_writer = BufWriter::new(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&file_name)?,
                );
                let mut heap: BinaryHeap<(Reverse<String>, usize, Option<String>)> =
                    BinaryHeap::new();
                for (i, reader) in low_readers.iter_mut().enumerate() {
                    if let Some((k, v)) = Self::get_kv(reader) {
                        heap.push((Reverse(k), i, v));
                    }
                }
                let mut pre: Option<(Reverse<String>, usize, Option<String>)> = None;
                while !heap.is_empty() {
                    let tmp = heap.pop().unwrap();
                    if let Some((k, v)) = Self::get_kv(&mut low_readers[tmp.1]) {
                        heap.push((Reverse(k), i, v));
                    }
                    if pre.is_none() || (pre.is_some() && pre.as_ref().unwrap().0 != tmp.0) {
                        Self::write_kv(&mut high_writer, &((tmp.0).0), tmp.2.as_ref())?;
                        pre = Some(tmp);
                    }
                }
                fs::rename(&file_name, file_name.replace(".unfinished", ".db"))?;
                //删除合并完的文件
                for path in low_paths {
                    fs::remove_file(path)?;
                }
            }
        }
        return Ok(());
    }
}
