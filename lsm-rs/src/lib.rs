mod lsm;
mod store;
#[cfg(test)]
mod tests {
    use crate::lsm::Lsm;

    #[test]
    fn it_works() {
        let mut lsm = Lsm::init(16, "store");
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
    }
}
