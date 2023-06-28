use crate::ValueEnum;
use std::collections::HashMap;

use crate::{pb::soulbound_modules::v1::Hotdog, helpers::HotdogHelpers};

/// This function will take in a blur_trade hotdog
/// And return a hotdog with the keys:
///     - collection
///     - token_id
///     - price
///     - payment_token
pub fn blur_trade_to_nft_price(hotdog: &Hotdog) -> Result<Hotdog, &str> {
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

    match (buy, sell) {
        (ValueEnum::MapValue(buy_map), ValueEnum::MapValue(sell_map)) => {
            let collection = buy_map.keys.get("collection").unwrap().clone();
            let price = buy_map.keys.get("price").unwrap().clone();
            let payment_token = buy_map.keys.get("paymentToken").unwrap().clone();
            let token_id = sell_map.keys.get("tokenId").unwrap().clone();
            let name = ValueEnum::StringValue("NFTPrice".to_string());

            let mut output_map: HashMap<String, ValueEnum> = HashMap::new();
            output_map.insert("hotdog_name".to_string(), name.into());
            output_map.insert("collection".to_string(), collection.into());
            output_map.insert("price".to_string(), price.into());
            output_map.insert("payment_token".to_string(), payment_token.into());
            output_map.insert("token_id".to_string(), token_id.into());
            Ok(Hotdog::from(output_map))
        }
        _ => Err("buy and sell are not maps")
    }
}
