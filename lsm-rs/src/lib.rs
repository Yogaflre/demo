mod index;
mod log;
mod lsm;
mod memtable;
mod reader;
mod sstable;
mod writer;

#[cfg(test)]
mod tests {

    use std::{io, thread, time::Duration};

    use crate::lsm::Lsm;

    #[test]
    fn it_works() -> io::Result<()> {
        let mut lsm = Lsm::new("store", 2, 7, 1);
        lsm.insert(&"key1".to_string(), &"value1".to_string())?;
        lsm.insert(&"key2".to_string(), &"value2".to_string())?;
        lsm.insert(&"key3".to_string(), &"value3".to_string())?;

        lsm.remove(&"key1".to_string())?;
        lsm.insert(&"key3".to_string(), &"value3_changed".to_string())?;

        assert_eq!(lsm.get(&"key1".to_string()).unwrap(), None);
        assert_eq!(
            lsm.get(&"key3".to_string()).unwrap(),
            Some("value3_changed".to_string())
        );
        assert_eq!(
            lsm.get(&"key2".to_string()).unwrap(),
            Some("value2".to_string())
        );
        println!("-----done-----");
        thread::sleep(Duration::from_secs(120));
        return Ok(());
    }
}
