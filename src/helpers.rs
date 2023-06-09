use std::{collections::HashMap, str::from_utf8};

use crate::pb::soulbound_modules::v1::{key_value::Value, Hotdog, KeyValue};
use sha3::{self, Digest};
use substreams_ethereum::{block_view::LogView, pb::eth::v2::Log};

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

pub struct EventSignature {
    event_name: String,
    params: Vec<EventParam>,
}

impl EventSignature {
    /// Takes an input string of the form,
    /// (EventName&type_indexed?_name&type_indexed_name)
    pub fn from_str(input_string: &str) -> EventSignature {
        // remove the parens and split along commas
        let split = input_string.trim()[1..input_string.len() - 2]
            .split("&")
            .collect::<Vec<_>>();
        let event_name = split[0].trim().to_string();

        let mut params = vec![];

        for param in split[1..].iter() {
            let param = param.trim();

            let mut split = param.split("_").peekable();

            let param_type = match split.next() {
                Some("string") => EventParamType::String,
                Some("address") => EventParamType::Address,
                Some("uint256") => EventParamType::Uint256,
                _ => panic!("Invalid event param type"),
            };

            let indexed = match split.peek() {
                Some(&"indexed") => true,
                _ => false,
            };

            if indexed {
                split.next();
            }

            let param_name = match split.next() {
                Some(name) => name.to_string(),
                _ => panic!("Invalid event param name"),
            };

            params.push(EventParam {
                param_type,
                indexed,
                param_name,
            });
        }

        EventSignature { event_name, params }
    }

    pub fn get_event_signature(&self) -> String {
        let event_signature = format!(
            "{}({})",
            self.event_name,
            self.params
                .iter()
                .map(|p| p.param_type.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        event_signature
        //hasher.update(event_signature.as_bytes());
    }

    /// Gets the topic 0 for the event, which identifies the kind of event emitted.
    /// This is the keccak256 hash of the event signature.
    /// IE: `Transfer(address,address,uint256)`
    pub fn get_topic_0(&self) -> Vec<u8> {
        let mut hasher = sha3::Keccak256::new();
        let event_signature = self.get_event_signature();
        hasher.update(event_signature.as_bytes());
        let result = hasher.finalize().to_vec();
        result
    }

    pub fn matches_log(&self, log: &LogView) -> bool {
        let topic_0 = self.get_topic_0();
        log.topics()[0] == topic_0
    }
}

pub struct EventParam {
    param_type: EventParamType,
    indexed: bool,
    param_name: String,
}

pub enum EventParamType {
    String,
    Address,
    Uint256,
}

impl ToString for EventParamType {
    fn to_string(&self) -> String {
        match self {
            EventParamType::String => "string".to_string(),
            EventParamType::Address => "address".to_string(),
            EventParamType::Uint256 => "uint256".to_string(),
        }
    }
}

pub fn log_to_hotdog(log: &LogView, event_signature: &EventSignature) -> Hotdog {
    let mut map = HashMap::new();

    let topics = log.topics();
    let _data = log.data();

    let mut topic_index = 1;
    let mut _data_index = 0;

    for param in event_signature.params.iter() {
        if param.indexed {
            // the bytes value
            let value = &topics[topic_index];

            let decoded_value = match param.param_type {
                EventParamType::String => {
                    let value = from_utf8(&value).unwrap();
                    Value::StringValue(value.to_string())
                }
                EventParamType::Address => Value::ByteValue(value[12..].to_vec()),
                EventParamType::Uint256 => Value::Uint64Value(69), //TODO this isn't safe lol
            };
            map.insert(param.param_name.clone(), decoded_value);

            topic_index += 1;
        } else {
            todo!("Decode non-indexed params from log data")
        }
    }

    hashmap_to_hotdog(map)
}
