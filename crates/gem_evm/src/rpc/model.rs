use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use serde_serializers::{deserialize_biguint_from_hex_str, deserialize_biguint_from_option_hex_str, deserialize_u64_from_str_or_int};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub transactions: Vec<Transaction>,
    #[serde(deserialize_with = "deserialize_biguint_from_hex_str")]
    pub timestamp: BigUint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactionsIds {
    pub transactions: Vec<String>,
    #[serde(deserialize_with = "deserialize_biguint_from_hex_str")]
    pub timestamp: BigUint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub from: String,
    #[serde(deserialize_with = "deserialize_u64_from_str_or_int")]
    pub gas: u64,
    // pub gas_price: String,
    // pub max_priority_fee_per_gas: Option<String>,
    // pub max_fee_per_gas: Option<String>,
    pub hash: String,
    pub input: String,
    pub to: Option<String>,
    #[serde(deserialize_with = "deserialize_biguint_from_hex_str")]
    pub value: BigUint,
    // #[serde(rename = "type")]
    // pub transaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReciept {
    #[serde(deserialize_with = "deserialize_biguint_from_hex_str")]
    pub gas_used: BigUint,
    #[serde(deserialize_with = "deserialize_biguint_from_hex_str")]
    pub effective_gas_price: BigUint,
    #[serde(default, deserialize_with = "deserialize_biguint_from_option_hex_str")]
    pub l1_fee: Option<BigUint>,
    pub logs: Vec<Log>,
    pub status: String,
    pub block_number: String,
}

impl TransactionReciept {
    pub fn get_fee(&self) -> BigUint {
        let fee = self.gas_used.clone() * self.effective_gas_price.clone();
        if let Some(l1_fee) = self.l1_fee.clone() {
            return fee + l1_fee;
        }
        fee
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
}
