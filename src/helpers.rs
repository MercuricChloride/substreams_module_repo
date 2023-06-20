use std::{collections::HashMap, ops::Mul, str::from_utf8};
use ethereum_abi::Value;
use fancy_regex::Regex;

use crate::pb::soulbound_modules::v1::{Hotdog, Hotdogs, Map};
use crate::pb::soulbound_modules::v1::{value::Value as ValueEnum, Value as ValueStruct};
use sha3::{self, Digest};
use substreams::log::println;
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::{block_view::LogView, pb::eth::v2::Log};

pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

/// TODO This is pretty slow, I gotta update this
pub fn hotdog_to_hashmap(hotdog: &Hotdog) -> HashMap<String, ValueEnum> {
    let mut map:HashMap<String, ValueEnum> = HashMap::new();


    for (key, value) in hotdog.map.as_ref().unwrap().keys.iter() {
        map.insert(key.to_string(), value.value.clone().unwrap());
    }

    map.insert("hotdog_name".to_string(), ValueEnum::StringValue(hotdog.hotdog_name.clone()));

    map
}

/// TODO This is pretty slow, I gotta update this
pub fn hashmap_to_hotdog(map: HashMap<String, ValueEnum>) -> Hotdog {
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

pub fn add_tx_meta(
    map: &mut HashMap<String, ValueEnum>,
    log: &LogView,
    block_timestamp: &String,
    block_hash: &String,
    block_number: u64,
) {
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
) -> Hotdog {
    todo!("finish this");
    // let mut map = HashMap::new();

    // let topics = log.topics();
    // let data = log.data();
    // println(format!("data size: {:?}", data.len()));

    // let mut topic_index = 1;
    // let mut data_index = 0;

    // add_tx_meta(&mut map, log, block_timestamp, block_hash, block_number);

    // println(format!("param count: {}", event_signature.params.len()));

    // for param in event_signature.params.iter() {
    //     if param.indexed {
    //         println(format!("param is indexed: {:?}", param.param_name));
    //         // the bytes value
    //         let value = &topics[topic_index];

    //         let decoded_value = match param.param_type {
    //             EventParamType::String => {
    //                 let value = from_utf8(&value).unwrap();
    //                 ValueEnum::StringValue(value.to_string())
    //             }
    //             EventParamType::Bytes(_) => ValueEnum::StringValue(format_hex(&value)),
    //             EventParamType::Address => ValueEnum::StringValue(format_hex(&value[12..])),
    //             EventParamType::Uint(_) => {
    //                 ValueEnum::StringValue(BigInt::from_unsigned_bytes_be(value).to_string())
    //             }
    //             EventParamType::Int(_) => {
    //                 ValueEnum::StringValue(BigInt::from_signed_bytes_be(value).to_string())
    //             }
    //             EventParamType::Array(_, _) => {
    //                 unimplemented!("Indexed array types are not supported yet, go bug @blind_nabler to fix this")
    //             }
    //             EventParamType::Tuple(_) => {
    //                 unimplemented!("Indexed tuple types are not supported yet, go bug @blind_nabler to fix this")
    //             }
    //         };

    //         map.insert(param.param_name.clone(), decoded_value);

    //         topic_index += 1;
    //     } else {

    //         if is_dynamic(&param.param_type) {
    //             println(format!("dynamic param: {:?}", param.param_name));
    //             // ethereum event data for dynamic types is weird
    //             // the first 32 bytes is the position of the actual data
    //             let dynamic_offset = &data[data_index..data_index + 32];
    //             println(format!("dynamic_offset: {}", format_hex(dynamic_offset)));
    //             // BigInt containing the offset in bytes
    //             let dynamic_location = BigInt::from_unsigned_bytes_be(dynamic_offset);
    //             // now we need to convert that to a usize
    //             let dynamic_location = usize::from_str_radix(&dynamic_location.to_string(), 10).unwrap();
    //             println(format!("dynamic_location: {}", dynamic_location));

    //             //let data = &data[dynamic_location..];
    //             let size = get_param_size(&param.param_type, &data, dynamic_location);
    //             let start_index = dynamic_location + 32;
    //             let end_index = start_index + size;
    //             let decoded_value = get_decoded_param(&param.param_type, data, start_index, end_index);
    //             println(format!("decoded_value: {:?}", decoded_value));
    //             map.insert(param.param_name.clone(), decoded_value);
    //         } else {
    //             println(format!("static param: {:?}", param.param_name));
    //             let data = &data[data_index..];

    //             let size = get_param_size(&param.param_type, &data);

    //             let padding = 32 - size;

    //             // NOTE Am I off by one byte here?
    //             let bytes = &data[padding..32];

    //             let decoded_value = get_decoded_param(&param.param_type, bytes);
    //             println(format!("decoded_value: {:?}", decoded_value));

    //             map.insert(param.param_name.clone(), decoded_value);
    //         }

    //         data_index += 32;
    //         //data_index += size;
    //     }
    // }

    // map.insert("hotdog_name".to_string(), ValueEnum::StringValue(event_signature.event_name.clone()));

    // hashmap_to_hotdog(map)
}

pub fn param_value_to_value_enum(value: &Value) -> ValueEnum {
    match value {
        Value::Uint(uint, _) => ValueEnum::StringValue(uint.to_string()),
        Value::Int(int, _) => ValueEnum::StringValue(int.to_string()),
        Value::Address(address) => ValueEnum::StringValue(address.to_string()),
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
