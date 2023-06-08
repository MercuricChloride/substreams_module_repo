mod pb;

use pb::soulbound_modules::v1::{key_value, DynamicKeyValue, Foo, KeyValue};
use prost::{encoding::message::encode, Message};
use prost_types::{value::Kind, Value};
use substreams::{self, errors::Error as SubstreamError};
use substreams_ethereum::pb::eth::v2 as eth;

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
pub fn map_key_value(param: String, blk: eth::Block) -> Result<DynamicKeyValue, SubstreamError> {
    if blk.number % 2 == 0 {
        Ok(DynamicKeyValue {
            keys: vec![KeyValue {
                key: "foo".to_string(),
                value: Some(key_value::Value::StringValue(param)),
            }],
        })
    } else {
        Ok(DynamicKeyValue {
            keys: vec![KeyValue {
                key: "bar".to_string(),
                value: Some(key_value::Value::StringValue("asdflkjasdf".to_string())),
            }],
        })
    }
}
