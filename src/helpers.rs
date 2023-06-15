use std::{collections::HashMap, ops::Mul, str::from_utf8};
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
    let mut map = HashMap::new();

    let topics = log.topics();
    let data = log.data();

    let mut topic_index = 1;
    let mut data_index = 0;

    add_tx_meta(&mut map, log, block_timestamp, block_hash, block_number);

    for param in event_signature.params.iter() {
        if param.indexed {
            // the bytes value
            let value = &topics[topic_index];

            let decoded_value = match param.param_type {
                EventParamType::String => {
                    let value = from_utf8(&value).unwrap();
                    ValueEnum::StringValue(value.to_string())
                }
                EventParamType::Bytes(_) => ValueEnum::StringValue(format_hex(&value)),
                EventParamType::Address => ValueEnum::StringValue(format_hex(&value[12..])),
                EventParamType::Uint(_) => {
                    ValueEnum::StringValue(BigInt::from_unsigned_bytes_be(value).to_string())
                }
                EventParamType::Int(_) => {
                    ValueEnum::StringValue(BigInt::from_signed_bytes_be(value).to_string())
                }
                EventParamType::Array(_, _) => {
                    unimplemented!("Indexed array types are not supported yet, go bug @blind_nabler to fix this")
                }
                EventParamType::Tuple(_) => {
                    unimplemented!("Indexed tuple types are not supported yet, go bug @blind_nabler to fix this")
                }
            };
            map.insert(param.param_name.clone(), decoded_value);

            topic_index += 1;
        } else {
            let data = &data[data_index..];

            let size = get_param_size(&param.param_type, &data);

            let bytes = &data[..size];

            let decoded_value = get_decoded_param(&param.param_type, bytes);

            map.insert(param.param_name.clone(), decoded_value);

            data_index += size;
        }
    }

    map.insert("hotdog_name".to_string(), ValueEnum::StringValue(event_signature.event_name.clone()));

    hashmap_to_hotdog(map)
}

fn get_decoded_param(param_type: &EventParamType, bytes: &[u8]) -> ValueEnum {
    match &param_type {
        EventParamType::String => {
            let value = from_utf8(bytes).unwrap();
            ValueEnum::StringValue(value.to_string())
        }
        EventParamType::Bytes(_) => ValueEnum::StringValue(format_hex(bytes)),
        EventParamType::Address => ValueEnum::StringValue(format_hex(bytes)),
        EventParamType::Uint(_) => {
            ValueEnum::StringValue(BigInt::from_unsigned_bytes_be(bytes).to_string())
        }
        EventParamType::Int(_) => {
            ValueEnum::StringValue(BigInt::from_signed_bytes_be(bytes).to_string())
        }
        EventParamType::Tuple(params) => {
            let mut map: HashMap<String, ValueStruct> = HashMap::new();
            for param in params {
                let size = get_param_size(&param.param_type, bytes);
                let value = get_decoded_param(&param.param_type, &bytes[..size]);
                map.insert(param.param_name.clone(), ValueStruct { value: Some(value) });
            }
            ValueEnum::MapValue(Map { keys: map })
        }
        EventParamType::Array(param_type, _) => {
            let size_per_type = get_param_size(param_type, bytes);
            // using this hashmap as an array because of our hotdog type
            let mut array = HashMap::new();
            for i in 0..bytes.len() / size_per_type {
                let value = get_decoded_param(param_type, &bytes[i * size_per_type..]);
                array.insert(i.to_string(), ValueStruct { value: Some(value) });
            }
            ValueEnum::MapValue(Map { keys: array })
        }
    }
}

fn get_param_size(param_type: &EventParamType, data: &[u8]) -> usize {
    match param_type {
        EventParamType::String => {
            // the first 32 bytes contain the length
            let byte_string_size = &data[..32];
            usize::from_be_bytes(byte_string_size.try_into().unwrap())
        }
        EventParamType::Bytes(Some(size)) => *size,
        EventParamType::Bytes(None) => {
            // the first 32 bytes contain the length
            let byte_string_size = &data[..32];
            usize::from_be_bytes(byte_string_size.try_into().unwrap())
        }
        EventParamType::Address => 20 as usize,
        EventParamType::Uint(size) | EventParamType::Int(size) => *size,
        EventParamType::Tuple(params) => {
            params.iter().map(|p| get_param_size(&p.param_type, data)).sum()
        }
        EventParamType::Array(param_type, size) => {
            if let Some(size) = size {
                size * get_param_size(param_type, data)
            } else {
                // the first 32 bytes contain the length
                let byte_string_size = &data[..32];
                let size = usize::from_be_bytes(byte_string_size.try_into().unwrap());
                size * get_param_size(param_type, data)
            }
        }
    }
}
