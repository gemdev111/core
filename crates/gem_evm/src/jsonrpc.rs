use gem_jsonrpc::types::{JsonRpcRequest, JsonRpcRequestConvert};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gasPrice")]
    pub gas_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub data: String,
}

impl TransactionObject {
    pub fn new_call(to: &str, data: Vec<u8>) -> Self {
        Self {
            from: None,
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: None,
            data: format!("0x{}", hex::encode(data)),
        }
    }

    pub fn new_call_to_value(to: &str, value: &str, data: Vec<u8>) -> Self {
        Self {
            from: None,
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: Some(value.to_string()),
            data: format!("0x{}", hex::encode(data)),
        }
    }

    pub fn new_call_with_from(from: &str, to: &str, data: Vec<u8>) -> Self {
        Self {
            from: Some(from.to_string()),
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: None,
            data: format!("0x{}", hex::encode(data)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockParameter {
    // hexadecimal block number
    Number(&'static str),
    Latest,
    Earliest,
    Pending,
    Safe,
    Finalized,
}

impl From<&BlockParameter> for &'static str {
    fn from(val: &BlockParameter) -> Self {
        match val {
            BlockParameter::Number(val) => val,
            BlockParameter::Latest => "latest",
            BlockParameter::Earliest => "earliest",
            BlockParameter::Pending => "pending",
            BlockParameter::Safe => "safe",
            BlockParameter::Finalized => "finalized",
        }
    }
}

impl From<&BlockParameter> for serde_json::Value {
    fn from(val: &BlockParameter) -> Self {
        let str: &str = val.into();
        serde_json::Value::String(str.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum EthereumRpc {
    Call(TransactionObject, BlockParameter),
    EstimateGas(TransactionObject, BlockParameter),
    GasPrice,
    GetBalance(&'static str),
    GetTransactionReceipt(String),
    FeeHistory { blocks: u64, reward_percentiles: Vec<u64> },
    TraceRawTransaction(String),
}

impl EthereumRpc {
    pub fn method_name(&self) -> &'static str {
        match self {
            EthereumRpc::GasPrice => "eth_gasPrice",
            EthereumRpc::GetBalance(_) => "eth_getBalance",
            EthereumRpc::Call(_, _) => "eth_call",
            EthereumRpc::GetTransactionReceipt(_) => "eth_getTransactionReceipt",
            EthereumRpc::EstimateGas(_, _) => "eth_estimateGas",
            EthereumRpc::FeeHistory { .. } => "eth_feeHistory",
            EthereumRpc::TraceRawTransaction(_) => "trace_rawTransaction",
        }
    }
}

impl JsonRpcRequestConvert for EthereumRpc {
    fn to_req(&self, id: u64) -> JsonRpcRequest {
        let method = self.method_name();
        let params: Vec<Value> = match self {
            EthereumRpc::GasPrice => vec![],
            EthereumRpc::GetBalance(address) => {
                vec![json!(address)]
            }
            EthereumRpc::Call(tx, block) => {
                let value = serde_json::to_value(tx).unwrap();
                vec![value, block.into()]
            }
            EthereumRpc::GetTransactionReceipt(tx_hash) => {
                vec![json!(tx_hash)]
            }
            EthereumRpc::EstimateGas(tx, block) => {
                let value = serde_json::to_value(tx).unwrap();
                vec![value, block.into()]
            }
            EthereumRpc::FeeHistory { blocks, reward_percentiles } => {
                vec![
                    json!(blocks),
                    json!("latest"),
                    json!(reward_percentiles.iter().map(|x| json!(x)).collect::<Vec<_>>()),
                ]
            }
            EthereumRpc::TraceRawTransaction(raw_tx) => {
                vec![json!(raw_tx), json!(vec!["stateDiff"])]
            }
        };

        JsonRpcRequest::new(id, method, params.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_call() {
        let request = TransactionObject::new_call_with_from(
            "0x46340b20830761efd32832a74d7169b29feb9758",
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            vec![],
        );
        let encoded = serde_json::to_string(&request).unwrap();

        assert_eq!(
            encoded,
            r#"{"from":"0x46340b20830761efd32832a74d7169b29feb9758","to":"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48","data":"0x"}"#
        );
    }
}
