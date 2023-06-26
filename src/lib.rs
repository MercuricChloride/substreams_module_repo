pub mod helpers;
mod pb;

use std::{collections::HashMap, str::FromStr, fmt::LowerExp};

use substreams::{pb::substreams::store_delta::Operation, store::{StoreAddBigInt, StoreAdd, StoreGetBigInt, StoreGet, StoreSetBigInt}, log::println};
use helpers::{format_hex, hashmap_to_hotdog, hotdog_to_hashmap, param_value_to_value_enum, add_tx_meta, log_to_hotdog, update_tables};
use pb::{soulbound_modules::v1::{
    Hotdog, Hotdogs, value::Value as ValueEnum, Value as ValueStruct
}, sf::substreams::{v1::module::input::Store, rpc::v2::StoreDelta}};
use substreams::{self, errors::Error as SubstreamError, store::{StoreSetIfNotExistsInt64, StoreSetIfNotExists, StoreSetIfNotExistsBigInt, StoreNew, DeltaBigInt, Deltas}, scalar::BigInt};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth::v2 as eth;
use ethereum_abi::Abi;

// takes an input string of address&&abi*
#[substreams::handlers::map]
pub fn map_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

    if split.len() % 2 != 0 {
        for item in split {
            println(format!("item {:?}\n\n\n",item));
        }

        panic!("Every address needs an ABI");
    }

    let mut contract_info: HashMap<String, Abi> = HashMap::new();

    for (index, item) in split.iter().enumerate() {
        if index % 2 == 0 {
            continue;
        } else {
            let address = split[index - 1].to_lowercase();
            let abi_json = item;
            let abi = serde_json::from_str(abi_json).unwrap();
            contract_info.insert(address, abi);
        }
    }

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
            if let Some(abi) = contract_info.get(&emitter) {
                log_to_hotdog(&log, block_number, &block_timestamp, &block_hash, &abi)
            } else {
                None
            }
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

// sees who owns what and stores it in the store
#[substreams::handlers::store]
fn store_ownership_distribution(hotdogs: Hotdogs, s: StoreAddBigInt) {
    // the hotdogs will be transfer events
    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "Transfer" {
            continue;
        }
        let map = hotdog_to_hashmap(&hotdog);
        let from = map.get("from").unwrap().clone();
        let to = map.get("to").unwrap().clone();
        let log_index = map.get("log_index").unwrap().clone();
        match (from, to, log_index) {
            (ValueEnum::StringValue(from), ValueEnum::StringValue(to), ValueEnum::StringValue(log_index)) => {
                let log_index = log_index.parse::<u64>().unwrap();
                s.add(log_index, from, BigInt::from(-1));
                s.add(log_index, to, BigInt::from(1));
            }
            _ => {}
        }
    }
}

// #[substreams::handlers::map]
// fn ownership_distribution(s: Deltas<Store>) -> Result<Hotdogs, SubstreamError> {
//     let mut hotdogs: Vec<Hotdog> = vec![];
//     for (key, value) in s {
//         hotdogs.push(hotdog);
//     }
//     Ok(Hotdogs { hotdogs })
// }

// filter all orders by a specific address
#[substreams::handlers::map]
fn filter_blur_trades(param: String, hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let filtered_addresses: Vec<String> = param.split("&&").map(|address| address.to_lowercase()).collect::<Vec<_>>();

    let mut filtered_hotdogs: Vec<Hotdog> = vec![];

    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "OrdersMatched" {
            continue;
        }

        let map = hotdog_to_hashmap(&hotdog);

        let buy = match map.get("buy") {
            Some(buy) => buy.clone(),
            None => panic!("map does not contain a buy field {:?}", hotdog)
        };

        let sell = match map.get("sell") {
            Some(sell) => sell.clone(),
            None => panic!("map does not contain a sell field {:?}", map)
        };

        match (buy, sell) {
            (ValueEnum::MapValue(buy_map), ValueEnum::MapValue(sell_map)) => {
                let buy_collection = buy_map.keys.get("collection").unwrap().clone();
                let sell_collection = sell_map.keys.get("collection").unwrap().clone();
                match (buy_collection.into(), sell_collection.into()) {
                    (ValueEnum::StringValue(buy_collection), ValueEnum::StringValue(sell_collection)) => {
                        if filtered_addresses.contains(&buy_collection) || filtered_addresses.contains(&sell_collection) {
                            filtered_hotdogs.push(hotdog.clone());
                        }

                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }

    Ok(Hotdogs {
        hotdogs: filtered_hotdogs
    })
}

#[substreams::handlers::store]
pub fn store_unique_users(hotdogs: Hotdogs, s: StoreSetIfNotExistsBigInt) {
    for hotdog in hotdogs.hotdogs {
        let map = hotdog_to_hashmap(&hotdog);

        let from: ValueEnum = map.get("tx_from").unwrap().clone();
        let to = map.get("tx_to").unwrap().clone();

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
        update_tables(map, &mut tables, None, None);
    }

    Ok(tables.to_entity_changes())
}
