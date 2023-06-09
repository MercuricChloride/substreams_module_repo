pub mod helpers;
mod pb;

use std::collections::HashMap;

use helpers::{format_hex, hashmap_to_hotdog, hotdog_to_hashmap};
use pb::soulbound_modules::v1::{
    key_value::{self, Value},
    Foo, Hotdog, Hotdogs, KeyValue,
};
use substreams::{self, errors::Error as SubstreamError};
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

#[substreams::handlers::map]
pub fn map_key_value(param: String, blk: eth::Block) -> Result<Hotdog, SubstreamError> {
    if blk.number % 2 == 0 {
        Ok(Hotdog {
            keys: vec![KeyValue {
                key: "foo".to_string(),
                value: Some(key_value::Value::StringValue(param)),
            }],
        })
    } else {
        Ok(Hotdog {
            keys: vec![KeyValue {
                key: "bar".to_string(),
                value: Some(key_value::Value::StringValue("asdflkjasdf".to_string())),
            }],
        })
    }
}

#[substreams::handlers::map]
pub fn map_hotdog(param: String, hotdog: Hotdog) -> Result<Hotdog, SubstreamError> {
    let hotdog_hash = hotdog_to_hashmap(&hotdog);

    let mut output_hash: HashMap<String, Value> = HashMap::new();

    let keys: Vec<&str> = param.split("-").collect();

    for key in keys.iter() {
        if let Some(value) = hotdog_hash.get(key.clone()) {
            output_hash.insert(key.to_string(), value.clone());
        } else {
            println!("Key {:?} not found", key);
        }
    }

    Ok(hashmap_to_hotdog(output_hash))
}

// Example InputString
//contract_address-(EventName, type indexed? name, type indexed? name)

// returns a hotdog with those fields
#[substreams::handlers::map]
pub fn map_event(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

    let contract_address = split.first().unwrap().to_lowercase();

    let event_signature = EventSignature::from_str(*split.last().unwrap());

    let hotdogs: Vec<Hotdog> = blk
        .logs()
        .filter_map(|log| {
            let emitter = format_hex(log.address());
            // let mut map = HashMap::new();
            // map.insert(
            //     "param_address".to_string(),
            //     Value::StringValue(contract_address.clone()),
            // );
            // map.insert(
            //     "emitter".to_string(),
            //     Value::StringValue(format_hex(log.address())),
            // );
            // let hotdog = hashmap_to_hotdog(map);
            // Some(hotdog)
            if event_signature.matches_log(&log) && emitter == contract_address {
                Some(log_to_hotdog(&log, &event_signature))
            } else {
                None
            }
        })
        .collect();

    Ok(Hotdogs { hotdogs })
}
