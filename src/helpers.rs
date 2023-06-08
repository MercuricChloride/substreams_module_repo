use std::collections::HashMap;

use crate::pb::soulbound_modules::v1::{key_value::Value, Hotdog, KeyValue};

/// TODO This is pretty slow, I gotta update this
pub fn hotdog_to_hashmap(hotdog: &Hotdog) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    for kv in hotdog.keys.iter() {
        let key = &kv.key;
        let value = &kv.value;
        if let Some(value) = value {
            map.insert(key.clone(), value.clone());
        } else {
            println!("{:?} is empty", key);
        }
    }
    map
}

/// TODO This is pretty slow, I gotta update this
pub fn hashmap_to_hotdog(map: HashMap<String, Value>) -> Hotdog {
    let mut keys: Vec<KeyValue> = vec![];

    for (key, value) in map {
        keys.push(KeyValue {
            key: key.clone(),
            value: Some(value.clone()),
        });
    }

    Hotdog { keys }
}
