pub mod helpers;
mod pb;

use std::{collections::HashMap, str::FromStr};

use substreams::{pb::substreams::store_delta::Operation, store::{StoreAddBigInt, StoreAdd, StoreGetBigInt, StoreGet}, log::println};
use helpers::{format_hex, hashmap_to_hotdog, hotdog_to_hashmap, param_value_to_value_enum, add_tx_meta, log_to_hotdog, update_tables};
use pb::{soulbound_modules::v1::{
    Foo, Hotdog, Hotdogs, value::Value as ValueEnum, Value as ValueStruct
}, sf::substreams::v1::module::input::Store};
use substreams::{self, errors::Error as SubstreamError, store::{StoreSetIfNotExistsInt64, StoreSetIfNotExists, StoreSetIfNotExistsBigInt, StoreNew, DeltaBigInt, Deltas}, scalar::BigInt};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
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

#[substreams::handlers::map]
pub fn map_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
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

            log_to_hotdog(&log, block_number, &block_timestamp, &block_hash, &abi)
        })
        .collect();

    Ok(Hotdogs{ hotdogs })
}

// Takes in a param string of the form
// Transfer&&Approval
// Keeps all events that match the names in the param
#[substreams::handlers::map]
fn filter_events(param: String, hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let filtered_names: Vec<&str> = param.split("&&").collect::<Vec<_>>();
    let mut filtered_hotdogs: Vec<Hotdog> = vec![];
    for hotdog in hotdogs.hotdogs {
        if filtered_names.contains(&hotdog.hotdog_name.as_str()) {
            filtered_hotdogs.push(hotdog.clone());
        }
    }
    Ok(Hotdogs {
        hotdogs: filtered_hotdogs
    })
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

#[substreams::handlers::map]
pub fn graph_out(hotdogs: Hotdogs) -> Result<EntityChanges, SubstreamError> {

    let mut tables = Tables::new();

    for hotdog in hotdogs.hotdogs {
        let map = hotdog_to_hashmap(&hotdog);
        update_tables(map, &mut tables, None);
    }

    Ok(tables.to_entity_changes())
}
