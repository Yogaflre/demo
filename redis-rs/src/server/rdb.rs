use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    mem,
    sync::mpsc::Sender,
};

use crate::{
    common::error::Result,
    types::{dict::Dict, object::Object},
};

/*
 * TODO
 * 1.使用fork进程。copy on write，利用虚拟进程分页来充分利用资源
 * 2.过滤已过期的key
 * 3.优化持久化文件格式
 *
 * 线程复制（会增大内存开销）
 */
fn save(path: &str, dict: Dict<Object>, expires: Dict<i64>, sender: Sender<Result<bool>>) {
    let res_func = || -> Result<bool> {
        let temp_path = format!("{}/db.tmp", path);
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temp_path)?;
        let dict_bytes = bincode::serialize(&dict)?;
        file.write_all(&dict_bytes.len().to_ne_bytes())?;
        file.write_all(&dict_bytes)?;
        let expires_bytes = bincode::serialize(&expires)?;
        file.write_all(&expires_bytes.len().to_ne_bytes())?;
        file.write_all(&expires_bytes)?;
        fs::rename(temp_path, format!("{}/db.rdb", path))?;
        return Ok(true);
    };
    sender.send(res_func()).unwrap();
}

/*
 * TODO
 * 1.过滤已过期的key
 */
fn load(path: &str) -> Result<(Option<Dict<Object>>, Option<Dict<i64>>)> {
    let rdb_path = format!("{}/db.rdb", path);
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(rdb_path)?);
    let mut buf = vec![0; mem::size_of::<usize>()];
    reader.read_exact(&mut buf)?;
    //FIXME
    let dict: = bincode::deserialize(&l.as_bytes())?
    let mut tmp = String::new();
    reader.read_line(&mut tmp)?;
    println!("line:{}", tmp);
    let mut lines = reader.lines();
    let dict: Option<Dict<Object>> = match lines.next() {
        Some(Ok(l)) => {
            println!("line: {:?}", l);
            bincode::deserialize(&l.as_bytes())?
        }
        _ => None,
    };
    let expires: Option<Dict<i64>> = match lines.next() {
        Some(Ok(l)) => bincode::deserialize(&l.as_bytes())?,
        _ => None,
    };
    return Ok((dict, expires));
}

#[test]
fn tmp() {
    load("store").unwrap();
}

#[test]
fn rdb() {
    use crate::db::db::Db;
    use chrono::Local;

    let base_path = "store";

    let id = 0;
    let is_master = true;
    let mut db = Db::new(id, is_master, None, None);
    let kv = "demo";
    db.set(&kv, &kv).unwrap();
    // expire after 10s.
    let expire_time = Local::now().timestamp_millis() + 10000;
    db.set_expire_time(kv, &expire_time.to_string()).unwrap();
    let store_dict = db.dict.clone();
    let store_expires = db.expires.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        save(base_path, store_dict, store_expires, tx);
    });
    assert!(rx.recv().unwrap().unwrap());
    assert!(std::path::Path::exists(
        format!("{}/db.rdb", base_path).as_ref()
    ));

    let (dict, expires): (Option<Dict<Object>>, Option<Dict<i64>>) = load(base_path).unwrap();
    assert_eq!(db.dict.dict_size(), dict.unwrap().dict_size());
    assert_eq!(db.expires.dict_size(), expires.unwrap().dict_size());
}
