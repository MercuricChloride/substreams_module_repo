use std::str::FromStr;
use std::{collections::HashMap, ops::Mul, str::from_utf8};
use ethereum_abi::Value;
use fancy_regex::Regex;
use substreams_entity_change::tables::Tables;

use crate::pb::soulbound_modules::v1::{Hotdog, Hotdogs, Map};
use crate::pb::soulbound_modules::v1::{value::Value as ValueEnum, Value as ValueStruct};
use sha3::{self, Digest};
use substreams::log::println;
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::{block_view::LogView, pb::eth::v2::Log};

pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

pub trait HotdogHelpers {
    fn to_hashmap(&self) -> HashMap<String, ValueEnum>;
    fn from_hashmap(map: HashMap<String, ValueEnum>) -> Self;
}

impl From<Hotdog> for HashMap<String, ValueEnum> {
    fn from(hotdog: Hotdog) -> Self {
        let mut map:HashMap<String, ValueEnum> = HashMap::new();


        for (key, value) in hotdog.map.as_ref().unwrap().keys.iter() {
            map.insert(key.to_string(), value.value.clone().unwrap());
        }

        map.insert("hotdog_name".to_string(), ValueEnum::StringValue(hotdog.hotdog_name.clone()));

        map
    }
}

impl From<HashMap<String, ValueEnum>> for Hotdog {
    fn from(map: HashMap<String, ValueEnum>) -> Self {
        let mut new_map: HashMap<String, ValueStruct> = HashMap::new();

        let hotdog_name = if let ValueEnum::StringValue(name) = map.get("hotdog_name").unwrap().clone() {
            name
        } else {
            panic!("No hotdog_name in hashmap");
        };

        for (key, value) in map {
            if key == "hotdog_name" {
                continue;
            }
            new_map.insert(key.clone(), ValueStruct{ value: Some(value.clone()) });
        }

        Hotdog { hotdog_name, map: Some(Map {keys: new_map} )}
    }
}

impl HotdogHelpers for Hotdog {
    /// TODO This is pretty slow, I gotta update this
    fn to_hashmap(&self) -> HashMap<String, ValueEnum> {
        self.clone().into()
    }

    /// TODO This is pretty slow, I gotta update this
    fn from_hashmap(map: HashMap<String, ValueEnum>) -> Self {
        map.into()
    }
}

pub fn add_tx_meta(
    map: &mut HashMap<String, ValueEnum>,
    log: &LogView,
    block_timestamp: &String,
    block_hash: &String,
    block_number: u64,
) {
    map.insert(
        "tx_log_index".to_string(),
        ValueEnum::StringValue(log.index().to_string()),
    );
    map.insert(
        "tx_hash".to_string(),
        ValueEnum::StringValue(format_hex(&log.receipt.transaction.hash)),
    );
    map.insert(
        "tx_index".to_string(),
        ValueEnum::StringValue(log.receipt.transaction.index.to_string()),
    );
    map.insert(
        "tx_from".to_string(),
        ValueEnum::StringValue(format_hex(&log.receipt.transaction.from)),
    );
    map.insert(
        "tx_to".to_string(),
        ValueEnum::StringValue(format_hex(&log.receipt.transaction.to)),
    );
    let gas_used = log.receipt.transaction.gas_used;
    map.insert(
        "tx_gas_used".to_string(),
        ValueEnum::StringValue(gas_used.to_string()),
    );
    if let Some(gas_price) = &log.receipt.transaction.gas_price {
        let gas_price = BigInt::from_unsigned_bytes_be(&gas_price.bytes);
        map.insert(
            "tx_gas_price".to_string(),
            ValueEnum::StringValue(gas_price.to_string()),
        );
        map.insert(
            "tx_total_gas_price".to_string(),
            ValueEnum::StringValue(gas_price.mul(gas_used).to_string()),
        );
    }
    map.insert("block_number".to_string(), ValueEnum::Uint64Value(block_number));
    map.insert(
        "block_hash".to_string(),
        ValueEnum::StringValue(block_hash.clone()),
    );
    map.insert(
        "block_timestamp".to_string(),
        ValueEnum::StringValue(block_timestamp.clone()),
    );
}

pub fn log_to_hotdog(
    log: &LogView,
    block_number: u64,
    block_timestamp: &String,
    block_hash: &String,
    abi: &ethereum_abi::Abi,
) -> Option<Hotdog> {
    let mut map = HashMap::new();

    let topics = &log.topics().iter().map(|topic| {
        primitive_types::H256::from_slice(&topic[..])
    }).collect::<Vec<_>>();

    add_tx_meta(&mut map, log, block_timestamp, block_hash, block_number);

    if let Ok((event, params)) = &abi.decode_log_from_slice(&topics[..] , log.data()) {
        let decoded_params = params;
        let mut map: HashMap<String, ValueEnum> = HashMap::new();
        map.insert("hotdog_name".to_string(), ValueEnum::StringValue(event.name.clone()));
        add_tx_meta(&mut map, &log, &block_timestamp, &block_hash, block_number);

        for kv in decoded_params.iter() {
            let param = &kv.param;
            let value = param_value_to_value_enum(&kv.value);
            map.insert(param.name.clone(), value);
        }

        Some(map.into())
    } else {
        None
    }
}

pub fn param_value_to_value_enum(value: &Value) -> ValueEnum {
    match value {
        Value::Uint(uint, _) => ValueEnum::StringValue(uint.to_string()),
        Value::Int(int, _) => ValueEnum::StringValue(int.to_string()),
        Value::Address(address) => ValueEnum::StringValue(format!("{:?}",address)),
        Value::Bool(boolean) => ValueEnum::StringValue(boolean.to_string()),
        Value::FixedBytes(bytes) => ValueEnum::StringValue(format_hex(&bytes)),
        Value::FixedArray(array, _) => {
            let mut map = HashMap::new();
            for i in 0..array.len() {
                let value = &array[i];
                map.insert(i.to_string(), ValueStruct { value: Some(param_value_to_value_enum(&value))});
            }
            ValueEnum::MapValue(
                Map { keys: map }
            )
        }
        Value::String(string) => ValueEnum::StringValue(string.to_string()),
        Value::Bytes(bytes) => ValueEnum::StringValue(format_hex(&bytes)),
        Value::Array(array, _) => {
            let mut map = HashMap::new();
            for i in 0..array.len() {
                let value = &array[i];
                map.insert(i.to_string(), ValueStruct { value: Some(param_value_to_value_enum(&value))});
            }
            ValueEnum::MapValue(
                Map { keys: map }
            )
        }
        Value::Tuple(tuple_arr) => {
            let mut map = HashMap::new();
            for (name, value) in tuple_arr.iter() {
                map.insert(name.to_string(), ValueStruct { value: Some(param_value_to_value_enum(&value))});
            }
            ValueEnum::MapValue(
                Map { keys: map }
            )
        }
    }
}

fn create_id(map: &HashMap<String, ValueEnum>) -> String {
    let tx_hash = match map.get("tx_hash") {
        Some(tx_hash) => tx_hash,
        None => panic!("{:?}", map)
    };

    let tx_log_index = map.get("tx_log_index").unwrap();

    // the id will be of form tx_hash-log_index
    match (tx_hash, tx_log_index) {
        (ValueEnum::StringValue(tx_hash), ValueEnum::StringValue(tx_log_index)) => {
            format!("{}-{}", tx_hash, tx_log_index)
        }
        _ => panic!("tx_hash and tx_log_index must be strings")
    }
}

pub fn update_tables(map: HashMap<String, ValueEnum>, tables: &mut Tables, table_name: Option<&String>, id: Option<&String>) {
    let id = match id {
        Some(id) => id.to_string(),
        None => create_id(&map)
    };

    let table_name = if let Some(table_name) = table_name {
        table_name

    } else {
        let table_name = &map.get("hotdog_name").expect("hotdog_name must be present");

        if let ValueEnum::StringValue(string_value) = table_name {
            string_value
        } else {
            panic!("hotdog_name must be a string")
        }
    };

    let row = tables.update_row(table_name, &id);

    let mut maps = vec![];
    for (key, value) in &map {
        if key == "hotdog_name" {
            continue;
        }
        match value {
            ValueEnum::Int64Value(int_value) => {
                row.set(&key, *int_value);
            },
            ValueEnum::Uint64Value(uint_value) => {
                row.set(&key, *uint_value);
            },
            ValueEnum::StringValue(string_value) => {
                if let Ok(_) = BigInt::from_str(&string_value) {
                    row.set_bigint(&key, &string_value);
                } else {
                    row.set(&key, string_value);
                }
            }
            ValueEnum::MapValue(map_value) => {
                maps.push((key, map_value));
            }
        };
    }

    for (_, map_value) in maps {
        let map: HashMap<String, ValueEnum> = map_value.clone().into();

        update_tables(map, tables, Some(table_name), Some(&id));
    }
}

impl Into<ValueEnum> for ValueStruct {
    fn into(self) -> ValueEnum {
        match self.value {
            Some(value) => value,
            None => panic!("value must be present")
        }
    }
}

impl Into<HashMap<String, ValueEnum>> for Map {
    fn into(self) -> HashMap<String, ValueEnum> {
        self.keys.into_iter().map(|(key, value)| {
            (key, value.into())
        }).collect()
    }
}

trait UpdateTables {
    fn create_id(&self) -> String;
    fn update_tables(&self, tables: &mut Tables);
}

impl UpdateTables for Hotdog {
    fn create_id(&self) -> String {
        let map = &self.to_hashmap();
        let tx_hash = map.get("tx_hash").unwrap();
        let tx_log_index = map.get("tx_log_index").unwrap();

        // the id will be of form tx_hash-log_index
        match (tx_hash, tx_log_index) {
            (ValueEnum::StringValue(tx_hash), ValueEnum::StringValue(tx_log_index)) => {
                format!("{}-{}", tx_hash, tx_log_index)
            }
            _ => panic!("tx_hash and tx_log_index must be strings")
        }
    }

    fn update_tables(&self, tables: &mut Tables) {
        let map = self.to_hashmap();

        let id = self.create_id();
        let table_name = &self.hotdog_name;
        let row = tables.create_row(table_name, id);

        for (key, value) in map {
            match value {
                ValueEnum::Int64Value(int_value) => row.set(&key, int_value),
                ValueEnum::Uint64Value(uint_value) => row.set(&key, uint_value),
                ValueEnum::StringValue(string_value) => {
                    if let Ok(_) = BigInt::from_str(&string_value) {
                        row.set_bigint(&key, &string_value)
                    } else {
                        row.set(&key, string_value)
                    }
                }
                ValueEnum::MapValue(map_value) => todo!(),
            };
        }
    }
}
