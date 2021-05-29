mod lsm;
mod store;
#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use crate::lsm::Lsm;

    #[test]
    fn it_works() {
        let mut lsm = Lsm::init("store", 16, 2);
        lsm.insert(&"key1".to_string(), &"value1".to_string());
        lsm.insert(&"key2".to_string(), &"value2".to_string());
        lsm.insert(&"key3".to_string(), &"value3".to_string());

        lsm.remove(&"key1".to_string());
        lsm.insert(&"key3".to_string(), &"value3_changed".to_string());

        assert_eq!(lsm.get(&"key1".to_string()), None);
        assert_eq!(
            lsm.get(&"key3".to_string()),
            Some("value3_changed".to_string())
        );
        thread::sleep(Duration::from_secs(120));
    }
}
