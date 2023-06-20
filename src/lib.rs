pub mod helpers;
mod pb;

use std::{collections::HashMap, str::FromStr};

use substreams::{pb::substreams::store_delta::Operation, store::{StoreAddBigInt, StoreAdd, StoreGetBigInt, StoreGet}, log::println};
use helpers::{format_hex, hashmap_to_hotdog, hotdog_to_hashmap, param_value_to_value_enum, add_tx_meta};
use pb::{soulbound_modules::v1::{
    Foo, Hotdog, Hotdogs, value::Value as ValueEnum, Value as ValueStruct
}, sf::substreams::v1::module::input::Store};
use substreams::{self, errors::Error as SubstreamError, store::{StoreSetIfNotExistsInt64, StoreSetIfNotExists, StoreSetIfNotExistsBigInt, StoreNew, DeltaBigInt, Deltas}, scalar::BigInt};
use substreams_entity_change::pb::entity::EntityChange;
use substreams_ethereum::{block_view::LogView, pb::eth::v2 as eth};

#[substreams::handlers::map]
pub fn map_blocks(param: String, blk: eth::Block) -> Result<Foo, SubstreamError> {
    let target_block = param
        .parse::<u64>()
        .expect("map_block: error parsing param as u64");
    if blk.number == target_block {
        Ok(Foo {
            number: blk.number,
            thing: param.clone(),
        })
    } else {
        Ok(Foo::default())
    }
}

// #[substreams::handlers::map]
// pub fn map_key_value(param: String, blk: eth::Block) -> Result<Hotdog, SubstreamError> {
//     if blk.number % 2 == 0 {
//         Ok(Hotdog {
//             keys: vec![KeyValue {
//                 key: "foo".to_string(),
//                 value: Some(key_value::Value::StringValue(param)),
//             }],
//         })
//     } else {
//         Ok(Hotdog {
//             keys: vec![KeyValue {
//                 key: "bar".to_string(),
//                 value: Some(key_value::Value::StringValue("asdflkjasdf".to_string())),
//             }],
//         })
//     }
// }

// #[substreams::handlers::map]
// pub fn map_hotdog(param: String, hotdog: Hotdog) -> Result<Hotdog, SubstreamError> {
//     let hotdog_hash = hotdog_to_hashmap(&hotdog);

//     let mut output_hash: HashMap<String, Value> = HashMap::new();

//     let keys: Vec<&str> = param.split("-").collect();

//     for key in keys.iter() {
//         if let Some(value) = hotdog_hash.get(key.clone()) {
//             output_hash.insert(key.to_string(), value.clone());
//         } else {
//             println!("Key {:?} not found", key);
//         }
//     }

//     Ok(hashmap_to_hotdog(output_hash))
// }

// Example InputString
//contract_address&&(EventName&type_indexed_name&type_indexed_name)

// returns a hotdog with those fields
// #[substreams::handlers::map]
// pub fn map_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
//     let split: Vec<&str> = param.split("&&").collect();

//     let contract_address = split.first().unwrap().to_lowercase();

//     let event_signature = EventSignature::from_str(*split.last().unwrap());
//     println(format!("event_signature: {:?}", format_hex(&event_signature.get_topic_0())));
//     let block_hash = format_hex(&blk.hash);
//     let block_number = blk.number;
//     let block_timestamp = blk
//         .header
//         .clone()
//         .unwrap()
//         .timestamp
//         .unwrap()
//         .seconds
//         .to_string();

//     let hotdogs: Vec<Hotdog> = blk
//         .logs()
//         .filter_map(|log| {
//             let emitter = format_hex(log.address());
//             if event_signature.matches_log(&log) && emitter == contract_address {
//                 Some(log_to_hotdog(
//                     &log,
//                     &event_signature,
//                     block_number,
//                     &block_timestamp,
//                     &block_hash,
//                 ))
//             } else {
//                 None
//             }
//         })
//         .collect();

//     Ok(Hotdogs { hotdogs })
// }

#[substreams::handlers::map]
pub fn map_abi_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

    let contract_address = split.first().unwrap().to_lowercase();

    let abi_json = split.last().unwrap();

    let abi: ethereum_abi::Abi = serde_json::from_str(abi_json).unwrap();

    let block_hash = format_hex(&blk.hash);
    let block_number = blk.number;
    let block_timestamp = blk
        .header
        .clone()
        .unwrap()
        .timestamp
        .unwrap()
        .seconds
        .to_string();

    let hotdogs: Vec<Hotdog> = blk
        .logs()
        .filter_map(|log| {
            let emitter = format_hex(log.address());
            if emitter != contract_address {
                return None;
            }


            let topics = &log.topics().iter().map(|topic| {
                primitive_types::H256::from_slice(&topic[..])
            }).collect::<Vec<_>>();

            if let Ok((event, params)) = &abi.decode_log_from_slice(&topics[..] , log.data()) {
                let decoded_params = params.reader().by_index;
                let mut map: HashMap<String, ValueEnum> = HashMap::new();
                map.insert("hotdog_name".to_string(), ValueEnum::StringValue(event.name.clone()));
                add_tx_meta(&mut map, &log, &block_timestamp, &block_hash, block_number);

                for kv in decoded_params {
                    let param = &kv.param;
                    let value = param_value_to_value_enum(&kv.value);
                    map.insert(param.name.clone(), value);
                }

                Some(hashmap_to_hotdog(map))
            } else {
                None
            }
        })
        .collect();

    Ok(Hotdogs{ hotdogs })
}

#[substreams::handlers::store]
pub fn store_unique_users(hotdogs: Hotdogs, s: StoreSetIfNotExistsBigInt) {
    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "Transfer" {
            continue;
        }
        let map = hotdog_to_hashmap(&hotdog);

        let from: ValueEnum = map.get("from").unwrap().clone();
        let to = map.get("to").unwrap().clone();

        if let ValueEnum::StringValue(from) = from {
            s.set_if_not_exists(0, &from, &BigInt::one());
        }

        if let ValueEnum::StringValue(to) = to {
            s.set_if_not_exists(0, &to, &BigInt::one());
        }
    }
}

#[substreams::handlers::store]
pub fn count_unique_users(unique_users: Deltas<DeltaBigInt>, s: StoreAddBigInt) {
    for delta in unique_users.deltas {
        // we only want to add to the total user count if the user is new
        if let Operation::Create = delta.operation {
            s.add(0, "unique_user_count", BigInt::one());
        }
    }
}

#[substreams::handlers::map]
pub fn map_unique_users(user_count: StoreGetBigInt) -> Result<Hotdog, SubstreamError> {
    if let Some(user_count) = user_count.get_last("unique_user_count") {
        let mut map: HashMap<String, ValueEnum> = HashMap::new();
        map.insert("hotdog_name".to_string(), ValueEnum::StringValue("unique_user_count".to_string()));
        map.insert("unique_user_count".to_string(), ValueEnum::StringValue(user_count.to_string()));
        Ok(hashmap_to_hotdog(map))
    } else {
        Ok(Hotdog::default())
    }
}

// #[substreams::handlers::map]
// pub fn graph_out(hotdogs: Hotdogs) -> Result<EntityChange, SubstreamError> {
//     let split: Vec<&str> = param.split("&&").collect();

//     let contract_address = split.first().unwrap().to_lowercase();

//     let event_signature = EventSignature::from_str(*split.last().unwrap());
//     let block_hash = format_hex(&blk.hash);
//     let block_number = blk.number;
//     let block_timestamp = blk
//         .header
//         .clone()
//         .unwrap()
//         .timestamp
//         .unwrap()
//         .seconds
//         .to_string();

//     let hotdogs: Vec<Hotdog> = blk
//         .logs()
//         .filter_map(|log| {
//             let emitter = format_hex(log.address());
//             if event_signature.matches_log(&log) && emitter == contract_address {
//                 Some(log_to_hotdog(
//                     &log,
//                     &event_signature,
//                     block_number,
//                     &block_timestamp,
//                     &block_hash,
//                 ))
//             } else {
//                 None
//             }
//         })
//         .collect();

//     Ok(Hotdogs { hotdogs })
// }
