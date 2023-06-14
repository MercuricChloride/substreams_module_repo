pub mod helpers;
mod pb;

use std::collections::HashMap;

use substreams::{pb::substreams::store_delta::Operation, store::{StoreAddBigInt, StoreAdd, StoreGetBigInt, StoreGet}};
use helpers::{format_hex, hashmap_to_hotdog, hotdog_to_hashmap};
use pb::{soulbound_modules::v1::{
    key_value::{self, Value},
    Foo, Hotdog, Hotdogs, KeyValue,
}, sf::substreams::v1::module::input::Store};
use substreams::{self, errors::Error as SubstreamError, store::{StoreSetIfNotExistsInt64, StoreSetIfNotExists, StoreSetIfNotExistsBigInt, StoreNew, DeltaBigInt, Deltas}, scalar::BigInt};
use substreams_entity_change::pb::entity::EntityChange;
use substreams_ethereum::{block_view::LogView, pb::eth::v2 as eth};

use crate::helpers::{log_to_hotdog, EventSignature};

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
#[substreams::handlers::map]
pub fn map_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

    let contract_address = split.first().unwrap().to_lowercase();

    let event_signature = EventSignature::from_str(*split.last().unwrap());
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
            if event_signature.matches_log(&log) && emitter == contract_address {
                Some(log_to_hotdog(
                    &log,
                    &event_signature,
                    block_number,
                    &block_timestamp,
                    &block_hash,
                ))
            } else {
                None
            }
        })
        .collect();

    Ok(Hotdogs { hotdogs })
}

#[substreams::handlers::store]
pub fn store_unique_users(hotdogs: Hotdogs, s: StoreSetIfNotExistsBigInt) {
    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "Transfer" {
            continue;
        }
        let map = hotdog_to_hashmap(&hotdog);

        let from: Value = map.get("from").unwrap().clone();
        let to = map.get("to").unwrap().clone();

        if let Value::StringValue(from) = from {
            s.set_if_not_exists(0, &from, &BigInt::one());
        }

        if let Value::StringValue(to) = to {
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
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("hotdog_name".to_string(), Value::StringValue("unique_user_count".to_string()));
        map.insert("unique_user_count".to_string(), Value::StringValue(user_count.to_string()));
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
