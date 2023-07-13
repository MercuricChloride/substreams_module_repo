use crate::{ValueEnum, helpers::clone_prefix};
use std::collections::HashMap;
use substreams::scalar::{BigInt, BigDecimal};
use std::str::FromStr;

use crate::{pb::soulbound_modules::v1::Hotdog, helpers::HotdogHelpers};

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

fn wei_to_eth(wei: &str) -> String {
    BigInt::from_str(wei).unwrap().to_decimal(18).to_string()
}

fn blur_trade_to_nft_price(hotdog: &Hotdog) -> Result<Hotdog, &str> {
    let name = &hotdog.hotdog_name;
    if name != "OrdersMatched" {
        return Err("hotdog is not an OrdersMatched hotdog");
    }

    let map = hotdog.to_hashmap();

    let buy = match map.get("buy") {
        Some(buy) => buy.clone(),
        None => return Err(stringify!("map does not contain a buy field {:?}", hotdog)),
    };

    let sell = match map.get("sell") {
        Some(sell) => sell.clone(),
        None => return Err(stringify!("map does not contain a sell field {:?}", map)),
    };

    match (buy, sell) {
        (ValueEnum::MapValue(buy_map), ValueEnum::MapValue(sell_map)) => {
            let collection = buy_map.keys.get("collection").unwrap().clone();
            let price = buy_map.keys.get("price").unwrap().clone();
            let price_string: String = match price.value.clone().unwrap() {
                ValueEnum::StringValue(price_string) => price_string,
                _ => return Err("price is not a string"),
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

            clone_prefix(&map, &mut output_map, &"tx_".to_string());

            Ok(Hotdog::from(output_map))
        }
        _ => Err("buy and sell are not maps"),
    }
}

fn seaport_trade_to_nft_price(hotdog: &Hotdog) -> Result<Hotdog, &str> {
    let name = &hotdog.hotdog_name;
    if name != "OrderFulfilled" {
        return Err("hotdog is not an OrderFulfilled hotdog");
    }

    let map = hotdog.to_hashmap();

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
    clone_prefix(&map, &mut output_map, &"tx_".to_string());

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
