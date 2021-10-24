use std::{
    fs::{self, OpenOptions},
    io::{BufReader, Read, Write},
    mem,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{common::error::Result, db::db::Db};

/*
 * TODO
 * 1.使用fork进程。copy on write，利用虚拟进程分页来充分利用资源
 * 2.过滤已过期的key
 * 3.优化持久化文件格式
 *
 * 线程复制（会增大内存开销）
 */
pub fn save(db: Db, is_saving: Arc<AtomicBool>) {
    let res_func = || -> Result<()> {
        let store_dir = &db.store_dir;
        let temp_path = format!("{}/db.tmp", store_dir);
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp_path)?;
        let db_bytes = bincode::serialize(&db)?;
        file.write_all(&db_bytes.len().to_le_bytes())?;
        file.write_all(&db_bytes)?;
        fs::rename(temp_path, format!("{}/db.rdb", store_dir))?;
        return Ok(());
    };
    res_func().unwrap_or(());
    is_saving
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap();
}

/*
 * 1.过滤已过期的key
 */
pub fn load(path: &str) -> Result<Option<Db>> {
    let rdb_path_str = format!("{}/db.rdb", path);
    println!("[rdb] load db from {} ...", &rdb_path_str);
    let rdb_path: &Path = rdb_path_str.as_ref();
    if !rdb_path.exists() {
        println!("[rdb] no rdb file ...");
        return Ok(None);
    }
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(rdb_path)?);

    let mut buf = vec![0; mem::size_of::<usize>()];
    let db: Option<Db>;
    if let Ok(_) = reader.read_exact(&mut buf) {
        let dict_size: usize = bincode::deserialize(&buf)?;
        buf = vec![0; dict_size];
        reader.read_exact(&mut buf)?;
        db = Some(bincode::deserialize(&buf)?);
    } else {
        db = None;
    }

    return Ok(db);
}

#[test]
fn rdb() {
    use crate::db::db::Db;
    use chrono::Local;

    let store_dir = "store";
    let mut db = Db::new(store_dir.to_string(), None, None);
    let kv = "demo";
    db.set(&kv, &kv).unwrap();
    // expire after 10s.
    let expire_time = Local::now().timestamp_millis() + 10000;
    db.set_expire_time(kv, &expire_time.to_string()).unwrap();
    let db_cloned = db.clone();
    let is_saving = Arc::new(AtomicBool::new(true));
    let is_saving_clone = is_saving.clone();
    std::thread::spawn(move || {
        save(db_cloned, is_saving_clone);
    });
    while is_saving.load(Ordering::SeqCst) {}
    assert!(std::path::Path::exists(
        format!("{}/db.rdb", store_dir).as_ref()
    ));

    let db_loaded: Db = load(store_dir).unwrap().unwrap();
    assert_eq!(db.dict.dict_size(), db_loaded.dict.dict_size());
    assert_eq!(db.expires.dict_size(), db_loaded.expires.dict_size());
    fs::remove_file(format!("{}/db.rdb", store_dir)).unwrap();
}
