use serde::{Deserialize, Serialize};
use serde_serializers::deserialize_f64_from_str;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Coins {
    pub coins: Vec<Asset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Asset {
    pub coin_code: String,
    pub blockchains: Vec<Blockchain>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Blockchain {
    pub code: String,
    pub contract_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct OrderData<T> {
    pub order: T,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: String,
    pub checkout_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderRequest {
    pub account_reference: String,
    pub source: String,
    pub source_amount: String,
    pub target: String,
    pub blockchain: String,
    pub wallet_address: String,
    pub return_url_on_success: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prices {
    pub spot_price: String,
    pub prices: Vec<Price>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub spot_price_including_fee: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub network_fee: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub fee_amount: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub fiat_amount: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderDetails {
    pub id: String,
    pub status: String,
    pub coin_code: String,
    pub fiat_amount: f64,
    pub fiat_code: String,
    pub wallet_address: String,
    pub tx_hash: Option<String>,
    pub blockchain: Blockchain,
    pub fee: Option<f64>,
    pub payment_fee: Option<f64>,
    pub merchant_fee: Option<f64>,
    pub network_fee: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Webhook {
    pub order_id: String,
}
