
# Table of Contents

1.  [Tasks <code>[0/5]</code>](#org960cc3f)
2.  [Notes](#org8026c38)
    1.  [Hotdog &ldquo;types&rdquo;](#hotdog_types)
    2.  [Hardcoded Modules](#org2e7de52)
        1.  [Why not just do the ABI<sub>GEN</sub> like the substreams example repo has?](#org573cb0e)
3.  [Protobufs](#org683fc5a)
    1.  [Hotdogs](#Hotdog)
        1.  [The reason for hotdogs](#orge49ea5a)
        2.  [Hotdog Code](#orgef3584d)
    2.  [Hotdog &ldquo;types&rdquo;](#org7e6e14b)
4.  [Substreams Yaml](#orgbea1075)
    1.  [Spec version and name](#org2325e34)
    2.  [Imports](#org33908c7)
    3.  [Protobuf definitions](#org3e304ce)
    4.  [Binary export](#org328d707)
    5.  [Params](#org47b6621)
    6.  [Modules](#orgafa505f)
5.  [build.rs](#build_script)
    1.  [Imports](#org7be7c17)
    2.  [Writing the abi Strings](#org34a95da)
6.  [helpers.rs](#orgc1d790f)
    1.  [Imports](#org825080d)
    2.  [Hotdog Helpers](#hotdog_helpers)
        1.  [Type Conversions](#orgcf9c0b3)
        2.  [Hotdog helpers trait](#orgf837bec)
        3.  [Misc Functions](#org3d6a370)
    3.  [General Helpers](#org1d2c89d)
        1.  [Format Hex](#org57895ba)
7.  [nft<sub>helpers.rs</sub>](#org325bb3a)
    1.  [Imports](#org2b66928)
    2.  [NFT Price &ldquo;type&rdquo;](#nft_price)
    3.  [Type Conversions](#orgc25716e)
        1.  [wei<sub>to</sub><sub>eth</sub>](#org895582e)
        2.  [blur<sub>trade</sub><sub>to</sub><sub>nft</sub><sub>price</sub>](#org18ce11f)
        3.  [seaport<sub>trade</sub><sub>to</sub><sub>nft</sub><sub>price</sub>](#org16b677a)
8.  [lib.rs](#org395a814)
    1.  [Imports and module declarations](#org90e56f5)
    2.  [Substreams Modules](#substream_modules)
        1.  [map<sub>events</sub>](#org1669c7c)
        2.  [filter<sub>events</sub>](#org90e6608)
        3.  [all<sub>blur</sub><sub>trades</sub>](#orgba6610d)
        4.  [filter<sub>blur</sub><sub>trades</sub>](#filter_blur_trades)
        5.  [blur<sub>trades</sub>](#orga4fd555)
        6.  [all<sub>seaport</sub><sub>trades</sub>](#orgde870f3)
        7.  [filter<sub>seaport</sub><sub>trades</sub>](#filter_blur_trades)
        8.  [seaport<sub>trades</sub>](#seaport_trades)
        9.  [graph<sub>out</sub>](#graph_out)
        10. [ownership<sub>distribution</sub>](#orga893e54)
        11. [unique<sub>users</sub>](#orgd322e1a)
        12. [etherscan<sub>overview</sub>](#orgf7af621)
        13. [gas<sub>guzzlers</sub>](#org635b9e0)



<a id="org960cc3f"></a>

# Tasks <code>[0/5]</code>

-   [ ] Add the superrare nft trade module
-   [ ] Figure out why the filter<sub>blur</sub><sub>trades</sub> module doesn&rsquo;t work with 1 input address
-   [ ] Add some new modules for DAOs
    I have some ideas regarding interesting modules for daos. Such as tracking ownership supply, as well as who is dumping what.
-   [ ] Make a plan on how to handle multi for multi trades with the seaport contracts
    Right now we are only handling trades from seaport if there is a single collection someone is trading for.
    We should probably also add support for many to many swaps
-   [ ] Add EOA support to the [etherscan<sub>overview</sub>](#orgf7af621) module


<a id="org8026c38"></a>

# Notes

This is a test of literate programming with lsp-mode in emacs

To edit something, execute (lsp-org) within a code block
To finish editing a block, execute (lsp-virtual-buffer-disconnect)

Seems like our ABI parser doesn&rsquo;t like contracts with custom errors. So we need to trim this from the abi.


<a id="hotdog_types"></a>

## Hotdog &ldquo;types&rdquo;

I am playing with this idea of hotdog &ldquo;types&rdquo;

Which are just a standard way of having a specific structure for these hotdogs to work with in a frontend.

The reason for this, is let&rsquo;s say you want to display NFT price data on a dashboard. If you get your values from say, seaport, blur, and superrare. You will have to extract meaningful price data from all 3 of these events in your dashboard.

If we instead have a hotdog that has keys already there for the price data, IE collection address, payment token, price, and token id, we don&rsquo;t have to worry about this.

It might make sense to just build these out as standalone protobufs, but for now I think it&rsquo;s fine as a hotdog.


<a id="org2e7de52"></a>

## Hardcoded Modules

Certain modules I think would benefit from hardcoding the ABI and maybe address inside.

Things like the blur and seaport modules.

To do this, within the [build script](#build_script), I will need to run some kind of ABI decoder. Or I could just write the ABI to an exported static constant variable.

I kind of like the exported static constant approach the most. Then decode them at runtime.


<a id="org573cb0e"></a>

### Why not just do the ABI<sub>GEN</sub> like the substreams example repo has?

The reason for this is because the binding generation they showed, does not support super complex event types, with say, nested tuples.

So it wont&rsquo; work for now, but maybe soon!


<a id="org683fc5a"></a>

# Protobufs


<a id="Hotdog"></a>

## Hotdogs

Hotdogs are a core part of how this whole module system works.

Helpers for working with hotdogs can be found [here](#hotdog_helpers)


<a id="orge49ea5a"></a>

### The reason for hotdogs

Substreams modules have to have a known protobuf type for their inputs and outputs.

However for the generalized event parsing module, we needed to be able to stream arbitrary events. IE Transfer or Approval etc.

So we needed a way for a module to output a dynamic type.

The name comes from the fact, you don&rsquo;t really know what is inside of a hotdog. But you can consume them all the same. Much like the data coming from these more dynamic modules!


<a id="orgef3584d"></a>

### Hotdog Code

    syntax = "proto3";
    package soulbound_modules.v1;
    
    message Value {
      oneof value {
        int32 int64_value = 1;
        uint64 uint64_value = 2;
        string string_value = 3;
        Map map_value = 4;
      }
    }
    
    message Map {
      map<string, Value> keys = 1;
    }
    
    message Hotdog {
      string hotdog_name = 1;
      Map map = 2;
    }
    
    message Hotdogs {
      repeated Hotdog hotdogs = 1;
    }


<a id="org7e6e14b"></a>

## Hotdog &ldquo;types&rdquo;

I am not sure exactly where to put this section. Because it is really more of a note than anything.

But I have this idea of &ldquo;hotdog types&rdquo;, where we have some unit structs, which have helpers going from and to this type. But they never aren&rsquo;t hotdogs.

The first example of this is the [Nft Price](#nft_price) type


<a id="orgbea1075"></a>

# Substreams Yaml


<a id="org2325e34"></a>

## Spec version and name

    specVersion: v0.1.0
    package:
      name: "soulbound_modules"
      version: v0.1.0


<a id="org33908c7"></a>

## Imports

    imports:
      entities_change: https://github.com/streamingfast/substreams-entity-change/releases/download/v0.2.1/substreams-entity-change-v0.2.1.spkg
      database_change: https://github.com/streamingfast/substreams-database-change/releases/download/v1.0.0/substreams-database-change-v1.0.0.spkg


<a id="org3e304ce"></a>

## Protobuf definitions

    protobuf:
      files:
        - soulbound.proto
      importPaths:
        - ./proto


<a id="org328d707"></a>

## Binary export

    binaries:
      default:
        type: wasm/rust-v1
        file: ./target/wasm32-unknown-unknown/release/soulbound_modules.wasm


<a id="org47b6621"></a>

## Params

    params:
      map_events: MAP_EVENTS_PARAMS
      filter_events: FILTER_EVENTS_PARAMS
      filter_blur_trades: FILTER_BLUR_TRADES_PARAMS
      etherscan_overview: ETHERSCAN_OVERVIEW_PARAMS
      #filter_blur_trades: "0x5Af0D9827E0c53E4799BB226655A1de152A425a5&&0x5Af0D9827E0c53E4799BB226655A1de152A425a5"


<a id="orgafa505f"></a>

## Modules

The module yaml config are exported within the module definition located at [8.2](#substream_modules)

    modules:
        - name: map_events
          kind: map
          inputs:
            - params: string
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: filter_events
          kind: map
          inputs:
            - params: string
            - map: map_events
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: all_blur_trades
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: filter_blur_trades
          kind: map
          inputs:
            - params: string
            - map: all_blur_trades
            #- map: map_events
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: blur_trades
          kind: map
          inputs:
            - map: filter_blur_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: all_seaport_trades
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: filter_seaport_trades
          kind: map
          inputs:
            - params: string
            - map: all_seaport_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: seaport_trades
          kind: map
          inputs:
            - map: filter_seaport_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: graph_out
          kind: map
          inputs:
            - map: map_events
          output:
            type: proto:substreams.entity.v1.EntityChanges
        - name: ownership_distribution
          kind: store
          updatePolicy: add
          valueType: bigint
          inputs:
            - map: map_events
        - name: store_unique_users
          kind: store
          updatePolicy: set_if_not_exists
          valueType: bigint
          inputs:
            - map: map_events
        - name: count_unique_users
          kind: store
          updatePolicy: add
          valueType: bigint
          inputs:
            - store: store_unique_users
              mode: deltas
        - name: map_unique_users
          kind: map
          inputs:
            - store: count_unique_users
              mode: get
          output:
            type: proto:soulbound_modules.v1.Hotdog
        - name: etherscan_overview
          kind: map
          inputs:
            - params: string
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs
        - name: gas_guzzlers
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="build_script"></a>

# build.rs

    <<build-script-imports>>
    
    fn main() {
    <<write-all-abi-strings>>
    }


<a id="org7be7c17"></a>

## Imports

    use std::fs;
    use std::io::Write;
    use std::path::Path;


<a id="org34a95da"></a>

## Writing the abi Strings

So we need to read each ABI, and write it to some constant file, and import it within the [lib.rs](#org395a814) file.

    // for each file within the abis/ dir, we need to write it as a constant string within src/abi_constants.rs file
    let path = Path::new("./abis");
    let files = fs::read_dir(path).unwrap();
    let mut abi_constants = String::new();
    for file in files  {
        let path = file.unwrap().path();
        let file_contents = fs::read_to_string(&path).unwrap();
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_uppercase();
        abi_constants.push_str(&format!("pub const {}: &str = r#\"{}\"#;\n", file_name, file_contents));
    }
    
    fs::write("./src/abi_constants.rs", abi_constants).unwrap();


<a id="orgc1d790f"></a>

# helpers.rs

A collection of helper functions to make life easier


<a id="org825080d"></a>

## Imports

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


<a id="hotdog_helpers"></a>

## Hotdog Helpers

The actual method for interacting with hotdogs can be really syntaxically gnarly.
So we have some helpers to work with them easier.

In general the best practice involves, converting a hotdog into a hashmap, working with that map, and then converting it back into a hotdog.

Also with values, the syntax can be gnarly, but most values impliment a .into() method for converting some value into a &ldquo;ValueEnum&rdquo; of the appropriate value.


<a id="orgcf9c0b3"></a>

### Type Conversions

1.  From Hotdog -> HashMap

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

2.  From Hashmap -> Hotdog

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

3.  Log -> hotdog

    Takes in a log and some other data and creates a hotdog from it
    
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

4.  ValueStruct into -> ValueEnum

    Conversion from a ValueStruct into a ValueEnum. Again naming doesn&rsquo;t feel great here but not sure how to make this better.
    
        impl Into<ValueEnum> for ValueStruct {
            fn into(self) -> ValueEnum {
                match self.value {
                    Some(value) => value,
                    None => panic!("value must be present")
                }
            }
        }

5.  Map into -> HashMap<String, ValueEnum>

    Another type conversion that helps with the syntax soup.
    
        impl Into<HashMap<String, ValueEnum>> for Map {
            fn into(self) -> HashMap<String, ValueEnum> {
                self.keys.into_iter().map(|(key, value)| {
                    (key, value.into())
                }).collect()
            }
        }


<a id="orgf837bec"></a>

### Hotdog helpers trait

Some similar type conversion helpers are present in this trait

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


<a id="org3d6a370"></a>

### Misc Functions

Just some more miscellaneous functions

1.  Add tx meta

    Adds the event log transaction metadata to a hotdog.
    
    This data is generally good to have
    
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

2.  Update Tables Trait and Impl

    The update tables trait is used to give the hotdog the ability to update postgres tables.
    
    This is used within the [8.2.9](#graph_out) module
    
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

3.  param<sub>value</sub><sub>to</sub><sub>value</sub><sub>enum</sub>

    This function converts a ethereum<sub>abi</sub>::Value into a value enum for use in a hotdog.
    
    I am not sure what better to name this, though the name doesn&rsquo;t feel great.
    
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


<a id="org1d2c89d"></a>

## General Helpers


<a id="org57895ba"></a>

### Format Hex

    pub fn format_hex(hex: &[u8]) -> String {
      format!("0x{}", Hex(hex).to_string())
    }


<a id="org325bb3a"></a>

# nft<sub>helpers.rs</sub>


<a id="org2b66928"></a>

## Imports

    use crate::ValueEnum;
    use std::collections::HashMap;
    use substreams::scalar::{BigInt, BigDecimal};
    use std::str::FromStr;
    
    use crate::{pb::soulbound_modules::v1::Hotdog, helpers::HotdogHelpers};


<a id="nft_price"></a>

## NFT Price &ldquo;type&rdquo;

Explanation of [hotdog types](#hotdog_types)

The purpose of this type is to have a common way to display an NFT trade in a dashboard, rather than handling events for each different nft marketplace event.

The way we are going to impliment any sort of &ldquo;types&rdquo; for hotdogs are as unit structs, with helper functions for converting specific other types or events into this &ldquo;type&rdquo;.

    /// A struct that allows conversions between a hotdog and a hotdog of type NftPrice
    /// NFTPrice contains:
    ///     - collection
    ///     - token_id
    ///     - price
    ///     - payment_token
    pub struct NftPrice;
    
    impl NftPrice {
        pub fn from_blur(hotdog: &Hotdog) -> Result<Hotdog, &str> {
            blur_trade_to_nft_price(hotdog)
        }
    
        pub fn from_seaport(hotdog: &Hotdog) -> Result<Hotdog, &str> {
            seaport_trade_to_nft_price(hotdog)
        }
    }


<a id="orgc25716e"></a>

## Type Conversions


<a id="org895582e"></a>

### wei<sub>to</sub><sub>eth</sub>

    fn wei_to_eth(wei: &str) -> String {
        BigInt::from_str(wei).unwrap().to_decimal(18).to_string()
    }


<a id="org18ce11f"></a>

### blur<sub>trade</sub><sub>to</sub><sub>nft</sub><sub>price</sub>

Converts a blur &ldquo;OrdersMatched&rdquo; event into an [7.2](#nft_price) hotdog.

    
    fn blur_trade_to_nft_price(hotdog: &Hotdog) -> Result<Hotdog, &str> {
        let name = &hotdog.hotdog_name;
        if name != "OrdersMatched" {
            return Err("hotdog is not an OrdersMatched hotdog");
        }
    
        let map = hotdog.to_hashmap();
    
        let buy = match map.get("buy") {
            Some(buy) => buy.clone(),
            None => return Err(stringify!("map does not contain a buy field {:?}", hotdog))
        };
    
        let sell = match map.get("sell") {
            Some(sell) => sell.clone(),
            None => return Err(stringify!("map does not contain a sell field {:?}", map))
        };
    
        let block_number = match map.get("block_number") {
            Some(block_number) => block_number.clone(),
            None => return Err(stringify!("map does not contain a block_number field {:?}", map))
        };
    
        match (buy, sell) {
            (ValueEnum::MapValue(buy_map), ValueEnum::MapValue(sell_map)) => {
                let collection = buy_map.keys.get("collection").unwrap().clone();
                let price = buy_map.keys.get("price").unwrap().clone();
                let price_string: String = match price.value.clone().unwrap() {
                    ValueEnum::StringValue(price_string) => price_string,
                    _ => return Err("price is not a string")
                };
    
                let price_in_eth = wei_to_eth(&price_string);
                let price_in_eth = ValueEnum::StringValue(price_in_eth);
                let payment_token = buy_map.keys.get("paymentToken").unwrap().clone();
                let token_id = sell_map.keys.get("tokenId").unwrap().clone();
                let name = ValueEnum::StringValue("NFTPrice".to_string());
    
                let mut output_map: HashMap<String, ValueEnum> = HashMap::new();
                output_map.insert("hotdog_name".to_string(), name.into());
                output_map.insert("collection".to_string(), collection.into());
                output_map.insert("price".to_string(), price.into());
                output_map.insert("price_in_eth".to_string(), price_in_eth);
    
                output_map.insert("payment_token".to_string(), payment_token.into());
                output_map.insert("token_id".to_string(), token_id.into());
                output_map.insert("block_number".to_string(), block_number.into());
                Ok(Hotdog::from(output_map))
            }
            _ => Err("buy and sell are not maps")
        }
    }


<a id="org16b677a"></a>

### seaport<sub>trade</sub><sub>to</sub><sub>nft</sub><sub>price</sub>

Converts a seaport trade into an [7.2](#nft_price) hotdog

    fn seaport_trade_to_nft_price(hotdog: &Hotdog) -> Result<Hotdog, &str> {
        let name = &hotdog.hotdog_name;
        if name != "OrderFulfilled" {
            return Err("hotdog is not an OrderFulfilled hotdog");
        }
    
        let map = hotdog.to_hashmap();
    
        let block_number = match map.get("block_number") {
            Some(block_number) => block_number.clone(),
            None => return Err(stringify!("map does not contain a block_number field {:?}", map))
        };
    
        let consideration = match map.get("consideration") {
            Some(consideration) => consideration.clone(),
            None => panic!("map does not contain a consideration field {:?}", hotdog)
        };
    
        let offer = match map.get("offer") {
            Some(offer) => offer.clone(),
            None => panic!("map does not contain a offer field {:?}", map)
        };
    
        let mut output_map: HashMap<String, ValueEnum> = HashMap::new();
        output_map.insert("hotdog_name".to_string(), ValueEnum::StringValue("NFTPrice".to_string()));
    
        // the whole thang goes like this:
        // user has an nft I want
        // I make an offer(s), an offer is a single item I am willing to give up
        // IE I offer 1 WETH for the nft
        // the consideration will be the NFT, and the offer will be the WETH
        // NOTE In our case, we are only going to track trades that are one item for one item
        match (consideration, offer) {
            (ValueEnum::MapValue(consideration), ValueEnum::MapValue(offer)) => {
                // the event field "offer" is an array of offers, the items spent
                // struct SpentItem {
                // enum ItemType itemType;
                // address token;
                // uint256 identifier;
                // uint256 amount;
                // }
                let mut nft_value: BigInt = BigInt::zero();
    
                for (index, offer) in offer.keys.iter() {
                    let value:ValueEnum = offer.clone().into();
    
                    let offer = match value {
                        ValueEnum::MapValue(map) => {
                            map
                        },
                        _ => panic!("offer is not a map!")
                    };
    
                    let offer = offer.keys;
    
                    let item_type = offer.get("itemType").unwrap().clone();
    
                    let item_type = match item_type.into() {
                        ValueEnum::StringValue(item_type) => item_type,
                        _ => panic!("item type is not a string!")
                    };
    
                    // if the item type isn't 2, it isn't an nft and we don't care about it
                    if item_type != "2" {
                        return Ok(Hotdog::default())
                    }
    
                    let collection:ValueEnum = offer.get("token").unwrap().clone().into();
                    let token_id:ValueEnum = offer.get("identifier").unwrap().clone().into();
    
                    if let Some(existing_collection) = output_map.insert("collection".to_string(), collection.clone()) {
                        match (existing_collection, collection) {
                            (ValueEnum::StringValue(existing_collection), ValueEnum::StringValue(collection)) => {
                                if existing_collection != collection {
                                    return Err("multiple collections in one hotdog");
                                }
                            },
                            _ => panic!("collection is not a string!")
                        }
                    };
                    if let Some(existing_token_id) = output_map.insert("token_id".to_string(), token_id.clone()) {
                        match (existing_token_id, token_id) {
                            (ValueEnum::StringValue(existing_token_id), ValueEnum::StringValue(token_id)) => {
                                if existing_token_id != token_id {
                                    return Err("multiple token_ids in one hotdog");
                                }
                            },
                            _ => panic!("token_id is not a string!")
                        }
                    };
                }
    
                // the event field "consideration" is an array of considerations, this is what is being received in the trade
                // struct ConsiderationItem {
                // enum ItemType itemType;
                // address token;
                // uint256 identifierOrCriteria;
                // uint256 endAmount;
                // uint256 startAmount;
                // address payable recipient;
                // }
                // enum ItemType {
                // NATIVE,
                // ERC20,
                // ERC721,
                // ERC1155,
                // ERC721_WITH_CRITERIA,
                // ERC1155_WITH_CRITERIA
                // }
    
                for (index, consideration) in consideration.keys.iter() {
                    let consideration: ValueEnum = consideration.clone().into();
    
                    let consideration = match consideration {
                        ValueEnum::MapValue(map) => {
                            map
                        },
                        _ => panic!("offer is not a map!")
                    };
    
                    let consideration = consideration.keys;
    
                    let item_type = consideration.get("itemType").unwrap().clone();
                    let item_type = match item_type.into() {
                        ValueEnum::StringValue(item_type) => item_type,
                        _ => panic!("item type is not a string!")
                    };
    
                    // if the item type is 2, it is an NFT and we don't want to track it
                    if item_type == "2" {
                        return Ok(Hotdog::default())
                    }
    
                    let token:ValueEnum = consideration.get("token").unwrap().clone().into();
                    let amount = consideration.get("amount").unwrap().clone();
                    let amount_string: String = match amount.value.clone().unwrap() {
                        ValueEnum::StringValue(amount_string) => amount_string,
                        _ => return Err("amount is not a string")
                    };
    
                    let amount = BigInt::from_str(&amount_string).unwrap();
                    nft_value = nft_value + amount;
    
                    if let Some(existing_token) = output_map.insert("payment_token".to_string(), token.clone()) {
                        match (existing_token, token) {
                            (ValueEnum::StringValue(existing_token), ValueEnum::StringValue(token)) => {
                                if existing_token != token {
                                    return Err("multiple payment_tokens in one hotdog");
                                }
                            },
                            _ => panic!("token is not a string!")
                        }
                    };
                }
                output_map.insert("price".to_string(), ValueEnum::StringValue(nft_value.to_string()));
                Ok(Hotdog::from(output_map))
            }
            _ => Ok(Hotdog::default())
        }
    }


<a id="org395a814"></a>

# lib.rs

The general file structure is as such:

    <<lib.rs-imports-and-modules>>
    
    <<substream-modules>>


<a id="org90e56f5"></a>

## Imports and module declarations

    mod abi_constants;
    pub mod helpers;
    pub mod nft_helpers;
    mod pb;

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


<a id="substream_modules"></a>

## Substreams Modules

Functions which represent the modules within the substream


<a id="org1669c7c"></a>

### map<sub>events</sub>

This module takes in a param string of the form

&ldquo;CONTRACT<sub>ADDRESS</sub>&&CONTRACT<sub>ABI</sub>&rdquo;

You can repeat this pattern, so long as every contract address has a abi that follows it.

The output of this module will be a [Hotdog](#Hotdog)

1.  Rust Code

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

2.  Yaml Definition

        - name: map_events
          kind: map
          inputs:
            - params: string
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="org90e6608"></a>

### filter<sub>events</sub>

This module takes in some hotdogs, which by default come from the map<sub>events</sub> module we defined above.

It also takes in a param string of the form:
&ldquo;EVENT<sub>TO</sub><sub>TRACK</sub>&rdquo;

This can be repeated where each event to track is split with &&

IE: &ldquo;Transfer&&Approval&rdquo;

1.  Rust Code

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

2.  Yaml Definition

        - name: filter_events
          kind: map
          inputs:
            - params: string
            - map: map_events
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="orgba6610d"></a>

### all<sub>blur</sub><sub>trades</sub>

This module is the hardcoded source of all blur trades. It will replace the map<sub>events</sub> input for [filter<sub>blur</sub><sub>trades</sub>](#filter_blur_trades)

It is super similar to [map<sub>events</sub>](#org1669c7c) in how it operates, just hardcoded is all :)

1.  Rust Code

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

2.  Yaml Definition

        - name: all_blur_trades
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="filter_blur_trades"></a>

### filter<sub>blur</sub><sub>trades</sub>

This module takes in an input from map<sub>events</sub>, and expects it to be tracking the blur marketplace contract.

This module also takes in a param string, which is a collection address or addresses to filter trades by.

IE: &ldquo;MILADY<sub>ADDRESS</sub>&&SOMETHING<sub>ELSE</sub>&rdquo; or &ldquo;MILADY<sub>ADDRESS</sub>&rdquo;

If you just want all trades from blur, just pass in an empty string.

1.  Rust Code

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

2.  Yaml Definition

        - name: filter_blur_trades
          kind: map
          inputs:
            - params: string
            - map: all_blur_trades
            #- map: map_events
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="orga4fd555"></a>

### blur<sub>trades</sub>

This module takes in filter<sub>blur</sub><sub>trades</sub> as an input, and converts it to the &ldquo;type&rdquo; [nft price](#nft_price)

1.  Rust Code

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

2.  Yaml Definition

        - name: blur_trades
          kind: map
          inputs:
            - map: filter_blur_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="orgde870f3"></a>

### all<sub>seaport</sub><sub>trades</sub>

This module is the hardcoded source of all blur trades. It will replace the map<sub>events</sub> input for [filter<sub>blur</sub><sub>trades</sub>](#filter_blur_trades)

It is super similar to [map<sub>events</sub>](#org1669c7c) in how it operates, just hardcoded is all :)

1.  Rust Code

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

2.  Yaml Definition

        - name: all_seaport_trades
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="filter_blur_trades"></a>

### filter<sub>seaport</sub><sub>trades</sub>

The same as the [filter<sub>blur</sub><sub>trades</sub>](#filter_blur_trades), however for seaport

1.  Rust Code

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

2.  Yaml Definition

        - name: filter_seaport_trades
          kind: map
          inputs:
            - params: string
            - map: all_seaport_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="seaport_trades"></a>

### seaport<sub>trades</sub>

Not done yet! But when it is, it will convert a seaport trade into a [nft price](#nft_price)

1.  Rust Code

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

2.  Yaml Definition

        - name: seaport_trades
          kind: map
          inputs:
            - map: filter_seaport_trades
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="graph_out"></a>

### graph<sub>out</sub>

This module converts a bunch of hotdogs to the appropriate entity changes within a postgres table.

The entity name according to the graphql schema should be the same as the hotdog name, otherwise an error will throw.

The module takes in input of map<sub>events</sub> by default, but works with any hotdog.

1.  Rust Code

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

2.  Yaml Definition

        - name: graph_out
          kind: map
          inputs:
            - map: map_events
          output:
            type: proto:substreams.entity.v1.EntityChanges


<a id="orga893e54"></a>

### ownership<sub>distribution</sub>

The idea behind this module is to see who owns what % of an nft collection.

Basically it&rsquo;s just a store that either adds or removes 1 from the from and to address in an nft transfer every time one gets emitted.

It&rsquo;s input should be a map<sub>events</sub>

1.  Rust Code

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

2.  Yaml Definition

        - name: ownership_distribution
          kind: store
          updatePolicy: add
          valueType: bigint
          inputs:
            - map: map_events


<a id="orgd322e1a"></a>

### unique<sub>users</sub>

This module counts how many unique users have interacted with a contract.

This needs a few modules in order to display nicely however.

1.  store<sub>unique</sub><sub>users</sub>

    stores the unique users in a store by address
    
    1.  Rust Code
    
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
    
    2.  Yaml Definition
    
            - name: store_unique_users
              kind: store
              updatePolicy: set_if_not_exists
              valueType: bigint
              inputs:
                - map: map_events

2.  count<sub>unique</sub><sub>users</sub>

    counts how many unique users have interacted with the store.
    
    The way this works is we just add 1 to the store value whenever the delta of the store<sub>unique</sub><sub>users</sub> module was a create operation, and do nothing otherwise.
    
    1.  Rust Code
    
            #[substreams::handlers::store]
            pub fn count_unique_users(unique_users: Deltas<DeltaBigInt>, s: StoreAddBigInt) {
                for delta in unique_users.deltas {
                    // we only want to add to the total user count if the user is new
                    if let Operation::Create = delta.operation {
                        s.add(0, "unique_user_count", BigInt::one());
                    }
                }
            }
    
    2.  Yaml definition
    
            - name: count_unique_users
              kind: store
              updatePolicy: add
              valueType: bigint
              inputs:
                - store: store_unique_users
                  mode: deltas

3.  map<sub>unique</sub><sub>users</sub>

    This reads from the last module, and emits the count within a hotdog
    
    1.  Rust Code
    
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
    
    2.  Yaml Definition
    
            - name: map_unique_users
              kind: map
              inputs:
                - store: count_unique_users
                  mode: get
              output:
                type: proto:soulbound_modules.v1.Hotdog


<a id="orgf7af621"></a>

### etherscan<sub>overview</sub>

This idea here is to build a module which mimics what you see on the etherscan overview page for a smart contract. As seen [here](https://etherscan.io/address/0x000000000000ad05ccc4f10045630fb830b95127)

This module will also include support for general EOA&rsquo;s, however for now I want to build for what people will mainly use.

This module takes in an input identical to [map<sub>events</sub>](#org1669c7c), the form of &ldquo;ADDRESS&&ABI&rdquo;

1.  Rust Code

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

2.  Yaml Definition

        - name: etherscan_overview
          kind: map
          inputs:
            - params: string
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs


<a id="org635b9e0"></a>

### gas<sub>guzzlers</sub>

This module just tracks how much gas each contract is using

1.  Rust Code

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

2.  Yaml Definition

        - name: gas_guzzlers
          kind: map
          inputs:
            - source: sf.ethereum.type.v2.Block
          output:
            type: proto:soulbound_modules.v1.Hotdogs

