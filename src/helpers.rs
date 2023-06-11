use std::{collections::HashMap, ops::Mul, str::from_utf8};

use crate::pb::soulbound_modules::v1::{key_value::Value, Hotdog, KeyValue};
use sha3::{self, Digest};
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::{block_view::LogView, pb::eth::v2::Log};

pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

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
        // remove the parens and split along &
        let split = input_string.trim()[1..input_string.len() - 1]
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
        if log.topics().len() == 0 {
            return false;
        }
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

fn add_tx_meta(
    map: &mut HashMap<String, Value>,
    log: &LogView,
    block_timestamp: &String,
    block_hash: &String,
    block_number: u64,
) {
    map.insert(
        "tx_hash".to_string(),
        Value::StringValue(format_hex(&log.receipt.transaction.hash)),
    );
    map.insert(
        "tx_index".to_string(),
        Value::StringValue(log.receipt.transaction.index.to_string()),
    );
    map.insert(
        "tx_from".to_string(),
        Value::StringValue(format_hex(&log.receipt.transaction.from)),
    );
    map.insert(
        "tx_to".to_string(),
        Value::StringValue(format_hex(&log.receipt.transaction.to)),
    );
    let gas_used = log.receipt.transaction.gas_used;
    map.insert(
        "tx_gas_used".to_string(),
        Value::StringValue(gas_used.to_string()),
    );
    if let Some(gas_price) = &log.receipt.transaction.gas_price {
        let gas_price = BigInt::from_unsigned_bytes_be(&gas_price.bytes);
        map.insert(
            "tx_gas_price".to_string(),
            Value::StringValue(gas_price.to_string()),
        );
        map.insert(
            "tx_total_gas_price".to_string(),
            Value::StringValue(gas_price.mul(gas_used).to_string()),
        );
    }
    map.insert("block_number".to_string(), Value::Uint64Value(block_number));
    map.insert(
        "block_hash".to_string(),
        Value::StringValue(block_hash.clone()),
    );
    map.insert(
        "block_timestamp".to_string(),
        Value::StringValue(block_timestamp.clone()),
    );
}

pub fn log_to_hotdog(
    log: &LogView,
    event_signature: &EventSignature,
    block_number: u64,
    block_timestamp: &String,
    block_hash: &String,
) -> Hotdog {
    let mut map = HashMap::new();

    let topics = log.topics();
    let data = log.data();

    let mut topic_index = 1;
    let mut data_index = 0;

    add_tx_meta(&mut map, log, block_timestamp, block_hash, block_number);

    for param in event_signature.params.iter() {
        if param.indexed {
            // the bytes value
            let value = &topics[topic_index];

            let decoded_value = match param.param_type {
                EventParamType::String => {
                    let value = from_utf8(&value).unwrap();
                    Value::StringValue(value.to_string())
                }
                EventParamType::Address => Value::StringValue(format_hex(&value[12..])),
                EventParamType::Uint256 => {
                    Value::StringValue(BigInt::from_signed_bytes_be(value).to_string())
                }
            };
            map.insert(param.param_name.clone(), decoded_value);

            topic_index += 1;
        } else {
            let data = &data[data_index..];

            let size = match param.param_type {
                EventParamType::String => {
                    // the first 32 bytes contain the length
                    let byte_string_size = &data[..32];
                    usize::from_be_bytes(byte_string_size.try_into().unwrap())
                }
                EventParamType::Address => 20 as usize,
                EventParamType::Uint256 => 32 as usize,
            };

            let bytes = &data[..size];

            let decoded_value = match param.param_type {
                EventParamType::String => {
                    let value = from_utf8(bytes).unwrap();
                    Value::StringValue(value.to_string())
                }
                EventParamType::Address => Value::StringValue(format_hex(bytes)),
                EventParamType::Uint256 => {
                    Value::StringValue(BigInt::from_unsigned_bytes_be(&bytes).to_string())
                }
            };
            map.insert(param.param_name.clone(), decoded_value);
            data_index += size;
        }
    }

    hashmap_to_hotdog(map)
}
