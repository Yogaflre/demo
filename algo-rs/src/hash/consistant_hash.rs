use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Node {
    id: String,
    values: HashMap<String, String>,
}

#[derive(Debug)]
struct ConsistantHash {
    // count = all node * replica
    virtual_nodes: BTreeMap<u64, Node>,
    // replica for peer node.
    replica: u8,
}

impl ConsistantHash {
    pub fn create(nodes: Vec<String>, replica: u8) -> ConsistantHash {
        let mut virtual_nodes: BTreeMap<u64, Node> = BTreeMap::new();
        for id in nodes.into_iter() {
            for i in 0..replica {
                let mut key = id.clone();
                key.push_str(&i.to_string());
                virtual_nodes.insert(
                    Self::hash_virtual_index(&key),
                    Node {
                        id: id.clone(),
                        values: HashMap::new(),
                    },
                );
            }
        }
        return ConsistantHash {
            virtual_nodes,
            replica,
        };
    }

    pub fn get(&mut self, k: String) -> Option<&String> {
        let node: &mut Node = self.find_first_node(&k);
        return node.values.get(&k);
    }

    pub fn add(&mut self, k: String, v: String) {
        let node: &mut Node = self.find_first_node(&k);
        node.values.insert(k, v);
    }

    pub fn delete(&mut self, k: String) -> Option<String> {
        let node: &mut Node = self.find_first_node(&k);
        return node.values.remove(&k);
    }

    pub fn add_node(&mut self, id: String) {
        for i in 0..self.replica {
            let mut key = id.clone();
            key.push_str(&i.to_string());
            self.virtual_nodes.insert(
                Self::hash_virtual_index(&key),
                Node {
                    id: id.clone(),
                    values: HashMap::new(),
                },
            );
        }
    }

    pub fn delete_node(&mut self, id: String) {
        for i in 0..self.replica {
            let mut target = id.clone();
            target.push_str(&i.to_string());
            let index = &Self::hash_virtual_index(&target);
            if let Some(node) = self.virtual_nodes.remove(&index) {
                let next: &mut Node = self.find_first_node_by_index(index + 1);
                for (k, v) in node.values {
                    next.values.insert(k, v);
                }
            }
        }
    }

    /*
     * get hash index (0 ~ 127)
     */
    fn hash_virtual_index(key: &String) -> u64 {
        let mut hasher = DefaultHasher::default();
        key.hash(&mut hasher);
        return hasher.finish() % (1 << 7);
    }

    fn find_first_node(&mut self, key: &String) -> &mut Node {
        let index = Self::hash_virtual_index(&key);
        return self.find_first_node_by_index(index);
    }

    fn find_first_node_by_index(&mut self, index: u64) -> &mut Node {
        if self.virtual_nodes.range(index..).next().is_some() {
            return self.virtual_nodes.range_mut(index..).next().unwrap().1;
        } else {
            return self
                .virtual_nodes
                .range_mut(0..)
                .next()
                .expect("virtual_nodes is empty!")
                .1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> ConsistantHash {
        let mut server =
            ConsistantHash::create(vec!["A".to_string(), "B".to_string(), "C".to_string()], 2);
        server.add("key1".to_string(), "1".to_string());
        server.add("key2".to_string(), "2".to_string());
        server.add("key3".to_string(), "3".to_string());
        server.add("key4".to_string(), "4".to_string());
        return server;
    }

    #[test]
    fn get() {
        let mut server = init();
        assert_eq!(server.get("key3".to_string()), Some(&"3".to_string()));
    }

    #[test]
    fn delete() {
        let mut server = init();
        assert_eq!(server.delete("key2".to_string()), Some("2".to_string()));
        assert_eq!(server.get("key2".to_string()), None);
    }

    #[test]
    fn add_node() {
        let mut server = init();
        server.add_node("D".to_string());
        server.add("key5".to_string(), "5".to_string());
        assert_eq!(server.get("key5".to_string()), Some(&"5".to_string()));
    }

    #[test]
    fn delete_node() {
        let mut server = init();
        let node_id = server.find_first_node(&"key2".to_string()).id.clone();
        println!("{:#?}", server);
        server.delete_node(node_id.clone());
        server.add_node(node_id.clone());
        // FIXME 当key转移到下一个节点后，重新上线被下线的节点，再查找需要遍历所有节点？
        assert_eq!(server.get("key2".to_string()), Some(&"2".to_string()));
        println!("{:#?}", server);
    }
}
