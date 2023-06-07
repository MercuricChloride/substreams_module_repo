mod pb;

use pb::soulbound_modules::v1::Foo;
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
