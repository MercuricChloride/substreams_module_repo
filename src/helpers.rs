// [[file:../Literate.org::helpers.rs/Imports][helpers.rs/Imports]]
use ethereum_abi::Value;
use fancy_regex::Regex;
use std::str::FromStr;
use std::{collections::HashMap, ops::Mul, str::from_utf8};
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::TransactionTrace;

use crate::pb::soulbound_modules::v1::{value_struct::ValueEnum, ValueStruct};
use crate::pb::soulbound_modules::v1::{Hotdog, Hotdogs, Map};
use sha3::{self, Digest};
use substreams::log::println;
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::{block_view::LogView, pb::eth::v2::Log};
// helpers.rs/Imports ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Type Conversions/From Hotdog -> HashMap][helpers.rs/Hotdog Helpers/Type Conversions/From Hotdog -> HashMap]]
impl From<Hotdog> for HashMap<String, ValueEnum> {
    fn from(hotdog: Hotdog) -> Self {
        let mut map: HashMap<String, ValueEnum> = HashMap::new();

        for (key, value_struct) in hotdog.map.as_ref().unwrap().kv.iter() {
            map.insert(key.to_string(), value_struct.value_enum.clone().unwrap());
        }

        map.insert(
            "hotdog_name".to_string(),
            ValueEnum::StringValue(hotdog.hotdog_name.clone()),
        );

        map
    }
}
// helpers.rs/Hotdog Helpers/Type Conversions/From Hotdog -> HashMap ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Type Conversions/From Hashmap -> Hotdog][helpers.rs/Hotdog Helpers/Type Conversions/From Hashmap -> Hotdog]]
impl From<HashMap<String, ValueEnum>> for Hotdog {
    fn from(map: HashMap<String, ValueEnum>) -> Self {
        let mut new_map: HashMap<String, ValueStruct> = HashMap::new();

        let hotdog_name =
            if let ValueEnum::StringValue(name) = map.get("hotdog_name").unwrap().clone() {
                name
            } else {
                panic!("No hotdog_name in hashmap");
            };

        for (key, value) in map {
            if key == "hotdog_name" {
                continue;
            }
            new_map.insert(
                key.clone(),
                ValueStruct {
                    value_enum: Some(value.clone()),
                },
            );
        }

        Hotdog {
            hotdog_name,
            map: Some(Map { kv: new_map }),
        }
    }
}
// helpers.rs/Hotdog Helpers/Type Conversions/From Hashmap -> Hotdog ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Type Conversions/Log -> hotdog][helpers.rs/Hotdog Helpers/Type Conversions/Log -> hotdog]]
pub fn log_to_hotdog(
    log: &LogView,
    block_number: u64,
    block_timestamp: &String,
    block_hash: &String,
    abi: &ethereum_abi::Abi,
) -> Option<Hotdog> {
    let mut map = HashMap::new();

    let topics = &log
        .topics()
        .iter()
        .map(|topic| primitive_types::H256::from_slice(&topic[..]))
        .collect::<Vec<_>>();

    add_tx_meta(
        &mut map,
        Some(log),
        log.receipt.transaction,
        block_timestamp,
        block_hash,
        block_number,
    );

    if let Ok((event, params)) = &abi.decode_log_from_slice(&topics[..], log.data()) {
        let decoded_params = params;
        let mut map: HashMap<String, ValueEnum> = HashMap::new();
        map.insert(
            "hotdog_name".to_string(),
            ValueEnum::StringValue(event.name.clone()),
        );
        add_tx_meta(
            &mut map,
            Some(log),
            log.receipt.transaction,
            block_timestamp,
            block_hash,
            block_number,
        );

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
// helpers.rs/Hotdog Helpers/Type Conversions/Log -> hotdog ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Type Conversions/ValueStruct into -> ValueEnum][helpers.rs/Hotdog Helpers/Type Conversions/ValueStruct into -> ValueEnum]]
impl Into<ValueEnum> for ValueStruct {
    fn into(self) -> ValueEnum {
        match self.value_enum {
            Some(value) => value,
            None => panic!("value must be present"),
        }
    }
}
// helpers.rs/Hotdog Helpers/Type Conversions/ValueStruct into -> ValueEnum ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Type Conversions/Map into -> HashMap<String, ValueEnum>][helpers.rs/Hotdog Helpers/Type Conversions/Map into -> HashMap<String, ValueEnum>]]
impl Into<HashMap<String, ValueEnum>> for Map {
    fn into(self) -> HashMap<String, ValueEnum> {
        self.kv
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect()
    }
}
// helpers.rs/Hotdog Helpers/Type Conversions/Map into -> HashMap<String, ValueEnum> ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Hotdog helpers trait][helpers.rs/Hotdog Helpers/Hotdog helpers trait]]
pub trait HotdogHelpers {
    fn to_hashmap(&self) -> HashMap<String, ValueEnum>;
    fn from_hashmap(map: HashMap<String, ValueEnum>) -> Self;
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
// helpers.rs/Hotdog Helpers/Hotdog helpers trait ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Misc Functions/Add tx meta][helpers.rs/Hotdog Helpers/Misc Functions/Add tx meta]]
pub fn add_tx_meta(
    map: &mut HashMap<String, ValueEnum>,
    log: Option<&LogView>,
    transaction: &TransactionTrace,
    block_timestamp: &String,
    block_hash: &String,
    block_number: u64,
) {
    if let Some(log) = log {
        map.insert(
            "tx_log_index".to_string(),
            ValueEnum::Uint64Value(log.index() as u64),
        );
    }

    map.insert(
        "tx_hash".to_string(),
        ValueEnum::StringValue(format_hex(&transaction.hash)),
    );
    map.insert(
        "tx_index".to_string(),
        ValueEnum::Uint64Value(transaction.index as u64),
    );
    map.insert(
        "tx_from".to_string(),
        ValueEnum::StringValue(format_hex(&transaction.from)),
    );
    map.insert(
        "tx_to".to_string(),
        ValueEnum::StringValue(format_hex(&transaction.to)),
    );
    let gas_used = transaction.gas_used;
    map.insert(
        "tx_gas_used".to_string(),
        ValueEnum::StringValue(gas_used.to_string()),
    );
    if let Some(gas_price) = &transaction.gas_price {
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
    map.insert(
        "tx_block_number".to_string(),
        ValueEnum::Uint64Value(block_number),
    );
    map.insert(
        "tx_block_hash".to_string(),
        ValueEnum::StringValue(block_hash.clone()),
    );
    map.insert(
        "tx_block_timestamp".to_string(),
        ValueEnum::Uint64Value(block_timestamp.parse::<u64>().unwrap()),
    );
}
// helpers.rs/Hotdog Helpers/Misc Functions/Add tx meta ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Misc Functions/clone_prefix][helpers.rs/Hotdog Helpers/Misc Functions/clone_prefix]]
pub fn clone_prefix(
    source_map: &HashMap<String, ValueEnum>,
    output_map: &mut HashMap<String, ValueEnum>,
    prefix: &String,
) {
    for (key, value) in source_map.iter() {
        if key.starts_with(prefix) {
            output_map.insert(key.clone(), value.clone());
        }
    }
}
// helpers.rs/Hotdog Helpers/Misc Functions/clone_prefix ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Misc Functions/Update Tables Trait and Impl][helpers.rs/Hotdog Helpers/Misc Functions/Update Tables Trait and Impl]]
pub trait UpdateTables {
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
            _ => panic!("tx_hash and tx_log_index must be strings"),
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
// helpers.rs/Hotdog Helpers/Misc Functions/Update Tables Trait and Impl ends here

// [[file:../Literate.org::helpers.rs/Hotdog Helpers/Misc Functions/param_value_to_value_enum][helpers.rs/Hotdog Helpers/Misc Functions/param_value_to_value_enum]]
pub fn param_value_to_value_enum(value: &Value) -> ValueEnum {
    match value {
        Value::Uint(uint, _) => ValueEnum::StringValue(uint.to_string()),
        Value::Int(int, _) => ValueEnum::StringValue(int.to_string()),
        Value::Address(address) => ValueEnum::StringValue(format!("{:?}", address)),
        Value::Bool(boolean) => ValueEnum::StringValue(boolean.to_string()),
        Value::FixedBytes(bytes) => ValueEnum::StringValue(format_hex(&bytes)),
        Value::FixedArray(array, _) => {
            let mut map = HashMap::new();
            for i in 0..array.len() {
                let value = &array[i];
                map.insert(
                    i.to_string(),
                    ValueStruct {
                        value_enum: Some(param_value_to_value_enum(&value)),
                    },
                );
            }
            ValueEnum::MapValue(Map { kv: map })
        }
        Value::String(string) => ValueEnum::StringValue(string.to_string()),
        Value::Bytes(bytes) => ValueEnum::StringValue(format_hex(&bytes)),
        Value::Array(array, _) => {
            let mut map = HashMap::new();
            for i in 0..array.len() {
                let value = &array[i];
                map.insert(
                    i.to_string(),
                    ValueStruct {
                        value_enum: Some(param_value_to_value_enum(&value)),
                    },
                );
            }
            ValueEnum::MapValue(Map { kv: map })
        }
        Value::Tuple(tuple_arr) => {
            let mut map = HashMap::new();
            for (name, value) in tuple_arr.iter() {
                map.insert(
                    name.to_string(),
                    ValueStruct {
                        value_enum: Some(param_value_to_value_enum(&value)),
                    },
                );
            }
            ValueEnum::MapValue(Map { kv: map })
        }
    }
}
// helpers.rs/Hotdog Helpers/Misc Functions/param_value_to_value_enum ends here

// [[file:../Literate.org::helpers.rs/General Helpers/Format Hex][helpers.rs/General Helpers/Format Hex]]
pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}
// helpers.rs/General Helpers/Format Hex ends here
