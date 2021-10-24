use std::collections::{BTreeMap, HashMap};

mod common;
mod db;
mod encoding;
mod server;
mod types;

fn main() {
    println!("Hello, world!");
    let mut map: BTreeMap<f32, i32> = BTreeMap::new();
    map.insert(1.1, 1);
}
