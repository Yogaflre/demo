use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
    panic,
    path::PathBuf,
};

use rb_tree::RBMap;

#[derive(Debug)]
pub struct Store {
    log: String,
    db: String,
    index: String,
    db_index: usize,
}
impl Store {
    pub fn init(path: &str) -> Self {
        let log = format!("{}/log", path);
        let db = format!("{}/db", path);
        let index = format!("{}/index", path);
        fs::create_dir_all(&log).unwrap();
        fs::create_dir_all(&db).unwrap();
        fs::create_dir_all(&index).unwrap();
        let dbs = fs::read_dir(&db).unwrap();

        return Store {
            log,
            db,
            index,
            db_index: dbs.count(),
        };
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

    pub fn store(&mut self, immutable_table: RBMap<String, Option<String>>) -> io::Result<()> {
        let mut db_file = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!("{}/{}.db", self.db, self.db_index))?,
        );
        for (k, v) in immutable_table.into_iter() {
            Self::write_kv(&mut db_file, &k, v.as_ref())?;
        }
        self.db_index += 1;
        fs::remove_file(format!("{}/save.log", self.log))?;
        return Ok(());
    }

    pub fn search_by_key(&self, key: &String) -> Option<String> {
        let mut paths = fs::read_dir(format!("{}/", self.db))
            .unwrap()
            .filter_map(|f| f.ok())
            .map(|f| f.path())
            .collect::<Vec<PathBuf>>();
        paths.sort_by(|a, b| b.as_path().cmp(a.as_path()));
        for path in paths {
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
        return None;
    }

    /*
     * 从日志恢复缓存
     */
    pub fn get_mem_table(&self) -> RBMap<String, Option<String>> {
        let mut map: RBMap<String, Option<String>> = RBMap::new();
        let mut reader = BufReader::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(format!("{}/cache.log", self.log))
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
        file: &mut BufWriter<File>,
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
            file.write_all(&(false as u8).to_le_bytes())?;
            file.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            file.write_all(&(val_bytes.len() as u8).to_le_bytes())?;
            file.write_all(key_bytes)?;
            file.write_all(val_bytes)?;
        } else {
            file.write_all(&(true as u8).to_le_bytes())?;
            file.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            file.write_all(key_bytes)?;
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
}
