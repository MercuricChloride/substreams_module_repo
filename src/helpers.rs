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

#[derive(Debug)]
pub struct EventSignature {
    event_name: String,
    params: Vec<EventParam>,
}

fn get_token(s: &str, pos: usize) -> Option<char> {
    s.chars().nth(pos)
}

fn is_eof(s: &str, pos: usize) -> bool {
    pos >= s.len()
}

impl EventSignature {
    // takes in an input string of the form,
    // (type_indexed?_name&(type_indexed_name))
    fn parse_params<'a>(input_string: &'a str, mut pos: usize, event_name: &'a mut Option<String>) -> (usize, Vec<EventParam>, &'a mut Option<String>) {
        let mut params: Vec<EventParam> = vec![];
        let mut current_word = String::new();
        //let mut event_name: Option<String> = None;

        while !is_eof(input_string, pos) {
            let token = get_token(input_string, pos).unwrap();
            pos += 1;

            match token {
                ']' => {
                    if current_word.contains("[") {
                        let mut num: Option<String> = None;
                        let mut last_token = current_word.pop().unwrap();

                        while last_token != '[' {
                            num = Some(format!("{}{}", last_token, num.unwrap_or("".to_string())));
                            last_token = current_word.pop().unwrap();
                        }

                        let param = params.pop().unwrap();

                        if let Some(num) = num {
                        params.push(
                            EventParam {
                                param_type: EventParamType::Array(Box::new(param.param_type), Some(num.parse::<usize>().unwrap())),
                                indexed: param.indexed,
                                param_name: param.param_name,
                            }
                        );
                        } else {
                        params.push(
                            EventParam {
                                param_type: EventParamType::Array(Box::new(param.param_type), None),
                                indexed: param.indexed,
                                param_name: param.param_name,
                            }
                        );
                        }
                    }
                }
                '&' => {
                    if !current_word.is_empty() {
                        let param = EventParam::from_str(&current_word);
                        // if we can parse it into a type then it's a param
                        if let Some(param_name) = param {
                            params.push(param_name);
                        } else {
                            // if we can't parse it into a type then it's the event name
                            // unless it starts with an underscore, which means its a tuples name and we should ignore it
                            if !current_word.starts_with("_") {
                                //println(&format!("setting event name to: {:?}", current_word));
                                *event_name = Some(current_word);
                            } else {
                                // otherwise it is the name of the last param
                                let param = params.pop().unwrap();
                                params.push(
                                    EventParam {
                                        param_type: param.param_type,
                                        indexed: param.indexed,
                                        param_name: current_word.trim_start_matches("_").to_string(),
                                    }
                                );
                            }
                        }
                        current_word = String::new();
                    }
                }
                '(' => {
                    // if the params are empty, its the start of the event signature
                    if params.is_empty() {
                        continue;
                    }
                    let (next_pos, list,_) = Self::parse_params(input_string, pos, event_name);
                    pos = next_pos;
                    params.push(
                        EventParam {
                            param_type: EventParamType::Tuple(list),
                            indexed: false,
                            param_name: "".to_string(),
                        }
                    );
                }
                ')' => {
                    if !current_word.is_empty() {
                        let param = EventParam::from_str(&current_word);
                        if let Some(param) = param {
                            params.push(param);
                        } else {
                            panic!("Invalid event param: {}", current_word);
                        }
                    }
                    return (pos, params, event_name);
                }
                _ => {
                    current_word.push(token);
                }
            }
        }

        if !current_word.is_empty() {
            println(&format!("params: {:?}", params));
            println(&format!("remaining word{}", current_word));
            let param = EventParam::from_str(&current_word);
            if let Some(param_name) = param {
                params.push(param_name);
            } else {
                // if we can't parse it into a type then it's the event name
                // unless it starts with an underscore, which means its a tuples name and we should ignore it
                if !current_word.starts_with("_") {
                    println(&format!("current_word: {:?}", current_word));
                    *event_name = Some(current_word);
                }
            }
        }

        (pos, params, event_name)
    }
    /// Takes an input string of the form,
    /// (EventName&type_indexed?_name&type_indexed_name)
    pub fn from_str(input_string: &str) -> EventSignature {
        let mut event_name = None;
        let (_, params, event_name) = EventSignature::parse_params(input_string, 0, &mut event_name);

        let event_name = if let Some(event_name) = event_name {
            event_name
        } else {
            panic!("No event name found");
        };


        EventSignature { event_name: event_name.to_string(), params }
    }

    pub fn get_event_signature(&self) -> String {
        let event_signature = format!(
            "{}({})",
            self.event_name,
            self.params
                .iter()
                .map(|p| {
                    p.param_type.to_string()
                    // if p.indexed {
                    //     //format!("indexed {}", p.param_type.to_string())
                    // } else {
                    //     p.param_type.to_string()
                    // }
                })
                .collect::<Vec<_>>()
                .join(",")
        );
        event_signature
    }

    /// Gets the topic 0 for the event, which identifies the kind of event emitted.
    /// This is the keccak256 hash of the event signature.
    /// IE: `Transfer(address,address,uint256)`
    pub fn get_topic_0(&self) -> Vec<u8> {
        let mut hasher = sha3::Keccak256::new();
        let event_signature = self.get_event_signature();
        hasher.update(event_signature.as_bytes());
        let result = hasher.finalize().to_vec();
        result
    }

    pub fn matches_log(&self, log: &LogView) -> bool {
        let topic_0 = self.get_topic_0();
        if log.topics().len() == 0 {
            return false;
        }
        log.topics()[0] == topic_0
    }
}

#[derive(Debug)]
pub struct EventParam {
    param_type: EventParamType,
    indexed: bool,
    param_name: String,
}

impl EventParam {
    // if this returns None, the param should be the name of the function
    pub fn from_str(str: &str) -> Option<Self> {
        let mut split = str.split("_").peekable();

        let type_signature = split.next();
        let mut param_type = match type_signature {
            Some(s) if s.starts_with("string") => EventParamType::String,
            Some(s) if s.starts_with("bytes") => {
                if let Ok(size) = s[5..].parse::<usize>() {
                    EventParamType::Bytes(Some(size))
                } else {
                    EventParamType::Bytes(None)
                }
            }
            Some(s) if s.starts_with("uint") => {
                if let Ok(size) = s[4..].parse::<usize>() {
                    EventParamType::Uint(size / 8)
                } else {
                    EventParamType::Uint(256 / 8)
                }
            }
            Some(s) if s.starts_with("address") => EventParamType::Address,
            Some(s) if s.starts_with("int") => {
                let size = s[3..].parse::<usize>().unwrap();
                EventParamType::Int(size / 8)
            }
            Some(s) if s.starts_with("(") => {
                EventParamType::Tuple(EventSignature::from_str(s).params)
            }
            _ => return None,
        };

        let re = Regex::new(r"\[(\d*)\]$").unwrap();

        if let Ok(Some(captures)) = re.captures(type_signature.unwrap()) {
            let size = captures.get(1).map_or(Ok(None), |m| m.as_str().parse::<usize>().map(Some)).unwrap();
            param_type = EventParamType::Array(Box::new(param_type), size);
        }

        let indexed = match split.peek() {
            Some(&"indexed") => true,
            _ => false,
        };

        if indexed {
            split.next();
        }

        let param_name = match split.next() {
            Some(name) => name.to_string(),
            _ => panic!("Invalid event param name"),
        };

        Some(EventParam {
            param_type,
            indexed,
            param_name,
        })
    }
}

#[derive(Debug)]
pub enum EventParamType {
    String,
    // Bytes with a fixed size or dynamically sized bytes
    Bytes(Option<usize>),
    Address,
    Tuple(Vec<EventParam>),
    Uint(usize),
    Int(usize),
    Array(Box<EventParamType>, Option<usize>),
}

impl ToString for EventParamType {
    fn to_string(&self) -> String {
        match self {
            EventParamType::String => "string".to_string(),
            EventParamType::Bytes(Some(size)) => format!("bytes{}", size),
            EventParamType::Bytes(None) => "bytes".to_string(),
            EventParamType::Address => "address".to_string(),
            EventParamType::Uint(size) => format!("uint{}", size * 8),
            EventParamType::Int(size) => format!("int{}", size * 8),
            EventParamType::Array(param_type, size) => {
                if let Some(size) = size {
                    return format!("{}[{}]", param_type.to_string(), size);
                } else {
                    return format!("{}[]", param_type.to_string());
                }
            }
            EventParamType::Tuple(params) => {
                format!("({})", params.iter().map(|p| p.param_type.to_string()).collect::<Vec<_>>().join(","))
            }
        }
    }
}

impl EventParamType {
    pub fn from_str(value: &str) -> Self {
        let normal_type = match value {
                s if s.starts_with("string") => EventParamType::String,
                s if s.starts_with("bytes") => {
                    if let Ok(size) = s[5..].parse::<usize>() {
                        EventParamType::Bytes(Some(size))
                    } else {
                        EventParamType::Bytes(None)
                    }
                }
                s if s.starts_with("uint") => {
                    if let Ok(size) = s[4..].parse::<usize>() {
                        EventParamType::Uint(size / 8)
                    } else {
                        EventParamType::Uint(256 / 8)
                    }
                }
                s if s.starts_with("address") => EventParamType::Address,
                s if s.starts_with("int") => {
                    if let Ok(size) = s[3..].parse::<usize>() {
                        EventParamType::Int(size / 8)
                    } else {
                        EventParamType::Int(256 / 8)
                    }
                }
                s if s.starts_with("(") => {
                    EventParamType::Tuple(EventSignature::from_str(s).params)
                }
                _ => panic!("Invalid event param type {:?}", value),
           };

        let re = Regex::new(r"\[(\d*)\]$").unwrap();

        if let Ok(Some(captures)) = re.captures(value) {
            let size = captures.get(1).map_or(None, |m| m.as_str().parse::<usize>().ok());
            EventParamType::Array(Box::new(normal_type), size)
        } else {
            normal_type
        }
    }
}

fn add_tx_meta(
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
    event_signature: &EventSignature,
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

fn get_param_offset(event_bytes: &[u8], data_index: usize) -> usize {
    let offset_bytes = &event_bytes[data_index..data_index + 32];
    let offset = BigInt::from_unsigned_bytes_be(offset_bytes);
    usize::from_str_radix(&offset.to_string(), 10).unwrap()
}

// this should be our main entrypoint for decoding params
// so when we iterate over the params in the event signature
// we can just call this function and it will return the decoded value
fn decode_param(event_bytes: &[u8], param_type: &EventParamType, data_index: usize) -> ValueEnum {
    todo!("finish this");
    // match &param_type {
    //     EventParamType::String => {
    //         // the actual location of the string in the event data
    //         let param_offset = get_param_offset(event_bytes, data_index);
    //         // gets the length of the string in bytes
    //         let param_length = get_param_size(&param_type);
    //         // the string data starts at the offset + 32 bytes (the first 32 bytes is the offset)
    //         let string_data_start = param_offset + 32;
    //         let value = from_utf8(&event_bytes[param_offset..param_offset + param_length]).unwrap();
    //     }
    // }
    // //if is_dynamic(param_type) {
    //     // get the size of the dynamic param
    // //} else {
    //     // get the size of the static param
    // //}
}

fn get_decoded_param(param_type: &EventParamType, bytes: &[u8], start_index: usize, stop_index: usize) -> ValueEnum {
    todo!("finish this");
    // match &param_type {
    //     EventParamType::String => {
    //         let value = from_utf8(&bytes[start_index..stop_index]).unwrap();
    //         ValueEnum::StringValue(value.to_string())
    //     }
    //     EventParamType::Bytes(_) => ValueEnum::StringValue(format_hex(&bytes[start_index..stop_index])),
    //     // NOTE, MAYBE?
    //     EventParamType::Address => ValueEnum::StringValue(format_hex(&bytes[start_index+12..stop_index])),
    //     EventParamType::Uint(_) => {
    //         ValueEnum::StringValue(BigInt::from_unsigned_bytes_be(&bytes[start_index..stop_index]).to_string())
    //     }
    //     EventParamType::Int(_) => {
    //         ValueEnum::StringValue(BigInt::from_signed_bytes_be(&bytes[start_index..stop_index]).to_string())
    //     }
    //     EventParamType::Tuple(params) => {
    //         let mut map: HashMap<String, ValueStruct> = HashMap::new();
    //         for (i, param) in params.iter().enumerate() {
    //             let mut size = get_param_size(&param.param_type, bytes, i*32);
    //             if size < 32 {
    //                 // we are doing this because ethereum doesn't pack types in event logs, so all things will occupy a 32 byte chunk
    //                 size = 32;
    //             };
    //             let start_index = i*32;
    //             let end_index = start_index + size;
    //             let value = get_decoded_param(&param.param_type, &bytes, start_index, end_index);
    //             map.insert(param.param_name.clone(), ValueStruct { value: Some(value) });
    //         }
    //         ValueEnum::MapValue(Map { keys: map })
    //     }
    //     EventParamType::Array(param_type, _) => {
    //         let mut size_per_type = get_param_size(param_type, bytes, start_index);
    //         if size_per_type < 32 {
    //             // we are doing this because ethereum doesn't pack types in event logs, so all things will occupy a 32 byte chunk
    //             size_per_type = 32;
    //         };

    //         // using this hashmap as an array because of our hotdog type
    //         let mut array = HashMap::new();
    //         for i in 0..bytes.len() / size_per_type {
    //             let start_index = i * size_per_type;
    //             let end_index = start_index + size_per_type;
    //             let value = get_decoded_param(param_type, bytes, start_index, end_index);
    //             array.insert(i.to_string(), ValueStruct { value: Some(value) });
    //         }
    //         ValueEnum::MapValue(Map { keys: array })
    //     }
    // }
}

/// The data param should be the entire event data, not just a slice of it
fn get_param_size(param_type: &EventParamType, data: &[u8], start_index: usize) -> usize {
    println(format!("getting size of : {:?}", param_type));
    let size = match param_type {
        EventParamType::String => {
            // the first 32 bytes contain the offset
            let offset = &data[start_index..32];
            // BigInt containing the offset in bytes
            let offset = BigInt::from_unsigned_bytes_be(offset);
            // now we need to convert that to a usize
            let offset = usize::from_str_radix(&offset.to_string(), 10).unwrap();

            // NOTE This was 64, shouldn't it be 32?
            let byte_string_size = &data[offset..32];
            usize::from_be_bytes(byte_string_size.try_into().unwrap())
        }
        EventParamType::Bytes(Some(size)) => *size,
        EventParamType::Bytes(None) => {
            // the first 32 bytes contain the offset
            let offset = &data[start_index..32];
            // BigInt containing the offset in bytes
            let offset = BigInt::from_unsigned_bytes_be(offset);
            // now we need to convert that to a usize
            let offset = usize::from_str_radix(&offset.to_string(), 10).unwrap();

            let byte_string_size = &data[offset..32];
            usize::from_be_bytes(byte_string_size.try_into().unwrap())
        }
        EventParamType::Tuple(params) => {
            let mut offset = 0;
            for param in params {
                let param_size = get_param_size(&param.param_type, &data, start_index + offset);
                if param_size < 32 {
                    // we are doing this because ethereum doesn't pack types in event logs, so all things will occupy a 32 byte chunk
                    offset += 32;
                } else {
                    offset += param_size;
                }
            }
            offset
            //params.iter().map(|p| get_param_size(&p.param_type, data)).sum()
        }
        EventParamType::Array(param_type, size) => {
            if let Some(size) = size {
                // if the array is sized, we can just multiply the size by the param size
                size * get_param_size(param_type, data, start_index)
            } else {
                // the first 32 bytes contain the offset
                let offset = &data[start_index..32];
                let offset = BigInt::from_unsigned_bytes_be(offset);
                let offset = usize::from_str_radix(&offset.to_string(), 10).unwrap();

                // the first 32 bytes from the offset contain the length
                let byte_string_size = &data[offset..32];
                println(format!("array length in hex: {}", format_hex(byte_string_size)));
                let size = BigInt::from_unsigned_bytes_be(byte_string_size);
                println(format!("array length: {}", size.to_string()));
                let size = usize::from_str_radix(&size.to_string(), 10).unwrap();
                let param_size = get_param_size(param_type, &data, offset+32);
                if param_size < 32 {
                    // we are doing this because ethereum doesn't pack types in event logs, so all things will occupy a 32 byte chunk
                    size * 32
                } else {
                    size * param_size
                }
            }
        }
        EventParamType::Address => 20 as usize,
        EventParamType::Uint(size) | EventParamType::Int(size) => *size,
    };
    println(format!("size: {}", size));
    size
}

fn is_dynamic(param_type: &EventParamType) -> bool {
    match param_type {
        EventParamType::Array(_, None) => true,
        EventParamType::String => true,
        EventParamType::Bytes(None) => true,
        EventParamType::Tuple(params) => params.iter().any(|p| is_dynamic(&p.param_type)),
        _ => false
    }
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
