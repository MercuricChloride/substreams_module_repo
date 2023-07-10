// [[file:../Literate.org::*General File Structure][General File Structure:1]]
// [[file:Literate.org::*General File Structure][]]
// [[file:Literate.org::lib.rs/Rust Modules][lib.rs/Rust Modules]]
mod abi_constants;
pub mod helpers;
pub mod nft_helpers;
mod pb;
// lib.rs/Rust Modules ends here
use ethereum_abi::Abi;
use helpers::{format_hex, log_to_hotdog, HotdogHelpers, UpdateTables};
use nft_helpers::NftPrice;
use pb::soulbound_modules::v1::{value::Value as ValueEnum, Hotdog, Hotdogs, Value as ValueStruct};
use std::collections::HashMap;
use substreams::{
    self,
    errors::Error as SubstreamError,
    scalar::BigInt,
    store::{DeltaBigInt, Deltas, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsBigInt},
};
use substreams::{
    log::println,
    pb::substreams::store_delta::Operation,
    store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt},
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth::v2 as eth;
// ends here

// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/map_events/Rust Code]]
// takes an input string of address&&abi*
#[substreams::handlers::map]
pub fn map_events(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

    if split.len() % 2 != 0 {
        for item in split {
            println(format!("item {:?}\n\n\n", item));
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

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/map_events/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/filter_events/Rust Code]]
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
        hotdogs: filtered_hotdogs,
    })
}
// lib.rs/Substreams Modules/filter_events/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/all_blur_trades/Rust Code]]
#[substreams::handlers::map]
pub fn all_blur_trades(blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let mut contract_info: HashMap<String, Abi> = HashMap::new();

    // Blur address
    let blur_address = "0x000000000000Ad05Ccc4F10045630fb830B95127"
        .to_lowercase()
        .to_string();
    let blur_abi = serde_json::from_str(abi_constants::BLUR).unwrap();

    contract_info.insert(blur_address, blur_abi);

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

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/all_blur_trades/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/filter_blur_trades/Rust Code]]
// filter all orders by a specific address
#[substreams::handlers::map]
fn filter_blur_trades(param: String, hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let filtered_addresses: Vec<String> = param
        .split("&&")
        .map(|address| address.to_lowercase())
        .collect::<Vec<_>>();

    if filtered_addresses.len() == 1 {
        return Ok(Hotdogs {
            hotdogs: hotdogs.hotdogs,
        });
    }

    let mut filtered_hotdogs: Vec<Hotdog> = vec![];

    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "OrdersMatched" {
            continue;
        }

        let map = &hotdog.to_hashmap();

        let buy = match map.get("buy") {
            Some(buy) => buy.clone(),
            None => panic!("map does not contain a buy field {:?}", hotdog),
        };

        let sell = match map.get("sell") {
            Some(sell) => sell.clone(),
            None => panic!("map does not contain a sell field {:?}", map),
        };

        match (buy, sell) {
            (ValueEnum::MapValue(buy_map), ValueEnum::MapValue(sell_map)) => {
                let buy_collection = buy_map.keys.get("collection").unwrap().clone();
                let sell_collection = sell_map.keys.get("collection").unwrap().clone();
                match (buy_collection.into(), sell_collection.into()) {
                    (
                        ValueEnum::StringValue(buy_collection),
                        ValueEnum::StringValue(sell_collection),
                    ) => {
                        if filtered_addresses.contains(&buy_collection)
                            || filtered_addresses.contains(&sell_collection)
                        {
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
        hotdogs: filtered_hotdogs,
    })
}
// lib.rs/Substreams Modules/filter_blur_trades/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/blur_trades/Rust Code]]
#[substreams::handlers::map]
pub fn blur_trades(hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let hotdogs = hotdogs
        .hotdogs
        .iter()
        .filter_map(|hotdog| match NftPrice::from_blur(hotdog) {
            Ok(hotdog) => Some(hotdog),
            _ => None,
        })
        .collect::<Vec<Hotdog>>();

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/blur_trades/Rust Code ends here
// [[file:../Literate.org::*General File Structure][lib.rs/Substreams Modules/all_seaport_trades/Rust Code]]
#[substreams::handlers::map]
pub fn all_seaport_trades(blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let mut contract_info: HashMap<String, Abi> = HashMap::new();

    // seaport address
    let seaport_address = "0x00000000000000ADc04C56Bf30aC9d3c0aAF14dC"
        .to_lowercase()
        .to_string();
    let seaport_abi = serde_json::from_str(abi_constants::SEAPORT).unwrap();

    contract_info.insert(seaport_address, seaport_abi);

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

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/all_seaport_trades/Rust Code ends here
// [[file:../Literate.org::*General File Structure][lib.rs/Substreams Modules/filter_seaport_trades/Rust Code]]
// filter all orders by a specific address
#[substreams::handlers::map]
fn filter_seaport_trades(param: String, hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let filtered_addresses: Vec<String> = param
        .split("&&")
        .map(|address| address.to_lowercase())
        .collect::<Vec<_>>();

    if filtered_addresses.len() == 1 {
        return Ok(Hotdogs {
            hotdogs: hotdogs.hotdogs,
        });
    }

    let mut filtered_hotdogs: Vec<Hotdog> = vec![];

    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "OrderFulfilled" {
            continue;
        }

        let map = &hotdog.to_hashmap();

        let consideration = match map.get("consideration") {
            Some(consideration) => consideration.clone(),
            None => panic!("map does not contain a consideration field {:?}", hotdog),
        };

        let offer = match map.get("offer") {
            Some(offer) => offer.clone(),
            None => panic!("map does not contain a offer field {:?}", map),
        };

        match (consideration, offer) {
            (ValueEnum::MapValue(consideration), ValueEnum::MapValue(offer)) => {
                // the event field "offer" is an array of offers, this is what is being purchased
                for (index, value) in offer.keys.iter() {
                    let value: HashMap<String, ValueEnum> = match value.clone().into() {
                        ValueEnum::MapValue(value) => value.into(),
                        _ => continue,
                    };
                    let collection = value.get("token").unwrap().clone();
                    match collection {
                        ValueEnum::StringValue(collection) => {
                            if filtered_addresses.contains(&collection) {
                                filtered_hotdogs.push(hotdog.clone());
                            }
                        }
                        _ => {}
                    }
                }
                // the event field "consideration" is an array of considerations, this is what is being sold to purchase the offer
                for (index, value) in consideration.keys.iter() {
                    let value: HashMap<String, ValueEnum> = match value.clone().into() {
                        ValueEnum::MapValue(value) => value.into(),
                        _ => continue,
                    };
                    let collection = value.get("token").unwrap().clone();
                    match collection {
                        ValueEnum::StringValue(collection) => {
                            if filtered_addresses.contains(&collection) {
                                filtered_hotdogs.push(hotdog.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }

    Ok(Hotdogs {
        hotdogs: filtered_hotdogs,
    })
}
// lib.rs/Substreams Modules/filter_seaport_trades/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/seaport_trades/Rust Code]]
#[substreams::handlers::map]
pub fn seaport_trades(hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let hotdogs = hotdogs
        .hotdogs
        .iter()
        .filter_map(|hotdog| match NftPrice::from_seaport(hotdog) {
            Ok(hotdog) => Some(hotdog),
            _ => None,
        })
        .collect::<Vec<Hotdog>>();

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/seaport_trades/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/graph_out/Rust Code]]
#[substreams::handlers::map]
pub fn graph_out(hotdogs: Hotdogs) -> Result<EntityChanges, SubstreamError> {
    let mut tables = Tables::new();

    for hotdog in hotdogs.hotdogs {
        hotdog.update_tables(&mut tables);
        //let map = hotdog.to_hashmap();
        //update_tables(map, &mut tables, None, None);
    }

    Ok(tables.to_entity_changes())
}
// lib.rs/Substreams Modules/graph_out/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/ownership_distribution/Rust Code]]
#[substreams::handlers::store]
fn store_ownership_distribution(hotdogs: Hotdogs, s: StoreAddBigInt) {
    // the hotdogs will be transfer events
    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "Transfer" {
            continue;
        }
        let map = hotdog.to_hashmap();
        let from = map.get("from").unwrap().clone();
        let to = map.get("to").unwrap().clone();
        let log_index = map.get("log_index").unwrap().clone();
        match (from, to, log_index) {
            (
                ValueEnum::StringValue(from),
                ValueEnum::StringValue(to),
                ValueEnum::StringValue(log_index),
            ) => {
                let log_index = log_index.parse::<u64>().unwrap();
                s.add(log_index, from, BigInt::from(-1));
                s.add(log_index, to, BigInt::from(1));
            }
            _ => {}
        }
    }
}
// lib.rs/Substreams Modules/ownership_distribution/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/unique_users/store_unique_users/Rust Code]]
#[substreams::handlers::store]
pub fn store_unique_users(hotdogs: Hotdogs, s: StoreSetIfNotExistsBigInt) {
    for hotdog in hotdogs.hotdogs {
        let map = hotdog.to_hashmap();

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
// lib.rs/Substreams Modules/unique_users/store_unique_users/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/unique_users/count_unique_users/Rust Code]]
#[substreams::handlers::store]
pub fn count_unique_users(unique_users: Deltas<DeltaBigInt>, s: StoreAddBigInt) {
    for delta in unique_users.deltas {
        // we only want to add to the total user count if the user is new
        if let Operation::Create = delta.operation {
            s.add(0, "unique_user_count", BigInt::one());
        }
    }
}
// lib.rs/Substreams Modules/unique_users/count_unique_users/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/unique_users/map_unique_users/Rust Code]]
#[substreams::handlers::map]
pub fn map_unique_users(user_count: StoreGetBigInt) -> Result<Hotdog, SubstreamError> {
    if let Some(user_count) = user_count.get_last("unique_user_count") {
        let mut map: HashMap<String, ValueEnum> = HashMap::new();
        map.insert(
            "hotdog_name".to_string(),
            ValueEnum::StringValue("unique_user_count".to_string()),
        );
        map.insert(
            "unique_user_count".to_string(),
            ValueEnum::StringValue(user_count.to_string()),
        );
        Ok(Hotdog::from_hashmap(map))
    } else {
        Ok(Hotdog::default())
    }
}
// lib.rs/Substreams Modules/unique_users/map_unique_users/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/etherscan_overview/Rust Code]]
// takes an input string of address&&abi*
#[substreams::handlers::map]
pub fn etherscan_overview(param: String, blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let split: Vec<&str> = param.split("&&").collect();

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
        .transaction_traces
        .iter()
        .filter_map(|transaction| {
            let from = format_hex(&transaction.from);
            let to = format_hex(&transaction.to);
            if transaction.input.len() < 4 {
                return None;
            }
            let method_signature = &transaction.input[0..4];

            if let Some(abi) = contract_info.get(&from) {
                let mut output_map: HashMap<String, ValueEnum> = HashMap::new();
                let functions = &abi.functions;
                let function = functions.iter().find(|function| {
                    let signature = function.method_id();
                    signature == method_signature
                });
                let signature = match function {
                    Some(function) => function.name.clone(),
                    None => format_hex(&method_signature),
                };
                // TODO add the tx meta stuff
                output_map.insert(
                    "hotdog_name".to_string(),
                    ValueEnum::StringValue("etherscan_overview".to_string()),
                );
                output_map.insert("from".to_string(), ValueEnum::StringValue(from));
                output_map.insert("to".to_string(), ValueEnum::StringValue(to));
                output_map.insert("method".to_string(), ValueEnum::StringValue(signature));
                Some(Hotdog::from(output_map))
            } else if let Some(abi) = contract_info.get(&to) {
                let mut output_map: HashMap<String, ValueEnum> = HashMap::new();

                let functions = &abi.functions;
                let function = functions.iter().find(|function| {
                    let signature = function.method_id();
                    signature == method_signature
                });
                let signature = match function {
                    Some(function) => function.name.clone(),
                    None => format_hex(&method_signature),
                };

                // TODO add the tx meta stuff
                output_map.insert(
                    "hotdog_name".to_string(),
                    ValueEnum::StringValue("etherscan_overview".to_string()),
                );
                output_map.insert("from".to_string(), ValueEnum::StringValue(from));
                output_map.insert("to".to_string(), ValueEnum::StringValue(to));
                output_map.insert("method".to_string(), ValueEnum::StringValue(signature));
                Some(Hotdog::from(output_map))
            } else {
                None
            }
        })
        .collect();

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/etherscan_overview/Rust Code ends here
// [[file:Literate.org::*General File Structure][lib.rs/Substreams Modules/gas_guzzlers/Rust Code]]
// takes an input string of address&&abi*
#[substreams::handlers::map]
pub fn gas_guzzlers(blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
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
        .transaction_traces
        .iter()
        .filter_map(|transaction| {
            let from = format_hex(&transaction.from);
            let to = format_hex(&transaction.to);
            if transaction.input.len() < 4 {
                return None;
            }
            let method_signature = format_hex(&transaction.input[0..4]);
            let gas = transaction.gas_used;

            let mut output_map: HashMap<String, ValueEnum> = HashMap::new();
            // TODO add the tx meta stuff
            output_map.insert(
                "hotdog_name".to_string(),
                ValueEnum::StringValue("etherscan_overview".to_string()),
            );
            output_map.insert("from".to_string(), ValueEnum::StringValue(from));
            output_map.insert("contract_address".to_string(), ValueEnum::StringValue(to));
            output_map.insert(
                "method".to_string(),
                ValueEnum::StringValue(method_signature),
            );
            output_map.insert(
                "block_hash".to_string(),
                ValueEnum::StringValue(block_hash.clone()),
            );
            output_map.insert(
                "block_number".to_string(),
                ValueEnum::Uint64Value(block_number),
            );
            output_map.insert(
                "block_timestamp".to_string(),
                ValueEnum::Uint64Value(block_timestamp.parse::<u64>().unwrap()),
            );
            output_map.insert("gas_used".to_string(), ValueEnum::Uint64Value(gas));
            Some(Hotdog::from(output_map))
        })
        .collect();

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/gas_guzzlers/Rust Code ends here
// General File Structure:1 ends here

// [[file:../Literate.org::lib.rs/Substreams Modules/all_seaport_trades/Rust Code][lib.rs/Substreams Modules/all_seaport_trades/Rust Code]]
#[substreams::handlers::map]
pub fn all_seaport_trades(blk: eth::Block) -> Result<Hotdogs, SubstreamError> {
    let mut contract_info: HashMap<String, Abi> = HashMap::new();

    // seaport address
    let seaport_address = "0x00000000000000ADc04C56Bf30aC9d3c0aAF14dC"
        .to_lowercase()
        .to_string();
    let seaport_abi = serde_json::from_str(abi_constants::SEAPORT).unwrap();

    contract_info.insert(seaport_address, seaport_abi);

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

    Ok(Hotdogs { hotdogs })
}
// lib.rs/Substreams Modules/all_seaport_trades/Rust Code ends here

// [[file:../Literate.org::lib.rs/Substreams Modules/filter_seaport_trades/Rust Code][lib.rs/Substreams Modules/filter_seaport_trades/Rust Code]]
// filter all orders by a specific address
#[substreams::handlers::map]
fn filter_seaport_trades(param: String, hotdogs: Hotdogs) -> Result<Hotdogs, SubstreamError> {
    let filtered_addresses: Vec<String> = param
        .split("&&")
        .map(|address| address.to_lowercase())
        .collect::<Vec<_>>();

    if filtered_addresses.len() == 1 {
        return Ok(Hotdogs {
            hotdogs: hotdogs.hotdogs,
        });
    }

    let mut filtered_hotdogs: Vec<Hotdog> = vec![];

    for hotdog in hotdogs.hotdogs {
        if hotdog.hotdog_name != "OrderFulfilled" {
            continue;
        }

        let map = &hotdog.to_hashmap();

        let consideration = match map.get("consideration") {
            Some(consideration) => consideration.clone(),
            None => panic!("map does not contain a consideration field {:?}", hotdog),
        };

        let offer = match map.get("offer") {
            Some(offer) => offer.clone(),
            None => panic!("map does not contain a offer field {:?}", map),
        };

        match (consideration, offer) {
            (ValueEnum::MapValue(consideration), ValueEnum::MapValue(offer)) => {
                // the event field "offer" is an array of offers, this is what is being purchased
                for (index, value) in offer.keys.iter() {
                    let value: HashMap<String, ValueEnum> = match value.clone().into() {
                        ValueEnum::MapValue(value) => value.into(),
                        _ => continue,
                    };
                    let collection = value.get("token").unwrap().clone();
                    match collection {
                        ValueEnum::StringValue(collection) => {
                            if filtered_addresses.contains(&collection) {
                                filtered_hotdogs.push(hotdog.clone());
                            }
                        }
                        _ => {}
                    }
                }
                // the event field "consideration" is an array of considerations, this is what is being sold to purchase the offer
                for (index, value) in consideration.keys.iter() {
                    let value: HashMap<String, ValueEnum> = match value.clone().into() {
                        ValueEnum::MapValue(value) => value.into(),
                        _ => continue,
                    };
                    let collection = value.get("token").unwrap().clone();
                    match collection {
                        ValueEnum::StringValue(collection) => {
                            if filtered_addresses.contains(&collection) {
                                filtered_hotdogs.push(hotdog.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }

    Ok(Hotdogs {
        hotdogs: filtered_hotdogs,
    })
}
// lib.rs/Substreams Modules/filter_seaport_trades/Rust Code ends here
