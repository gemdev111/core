use chrono::DateTime;
use num_bigint::BigUint;
use num_traits::Num;
use primitives::{chain::Chain, AssetId, StakeValidator, Transaction, TransactionState, TransactionType};

use super::model::{self, TransactionReceiptData, WitnessesList};
use crate::{address::TronAddress, rpc::model::BlockTransactions};

const TRANSFER_CONTRACT: &str = "TransferContract";
const TRIGGER_SMART_CONTRACT: &str = "TriggerSmartContract";

pub struct TronMapper;

impl TronMapper {
    pub fn map_transactions(chain: Chain, block: BlockTransactions, reciepts: Vec<TransactionReceiptData>) -> Vec<Transaction> {
        block
            .transactions
            .unwrap_or_default()
            .into_iter()
            .zip(reciepts.iter())
            .filter_map(|(transaction, receipt)| TronMapper::map_transaction(chain, transaction, receipt.clone()))
            .collect()
    }

    pub fn map_transaction(chain: Chain, transaction: model::Transaction, receipt: TransactionReceiptData) -> Option<Transaction> {
        if let (Some(value), Some(contract_result)) = (transaction.raw_data.contract.first().cloned(), transaction.ret.first().cloned()) {
            let state: TransactionState = if contract_result.contract_ret.clone() == "SUCCESS" {
                TransactionState::Confirmed
            } else {
                TransactionState::Failed
            };
            let fee = receipt.fee.unwrap_or_default().to_string();
            let created_at = DateTime::from_timestamp_millis(receipt.block_time_stamp)?;

            if value.contract_type == TRANSFER_CONTRACT && !transaction.ret.is_empty() {
                let from = TronAddress::from_hex(value.parameter.value.owner_address.unwrap_or_default().as_str()).unwrap_or_default();
                let to = TronAddress::from_hex(value.parameter.value.to_address.unwrap_or_default().as_str()).unwrap_or_default();

                let transaction = Transaction::new(
                    transaction.tx_id,
                    chain.as_asset_id(),
                    from,
                    to,
                    None,
                    TransactionType::Transfer,
                    state,
                    fee,
                    chain.as_asset_id(),
                    value.parameter.value.amount.unwrap_or_default().to_string(),
                    None,
                    None,
                    created_at,
                );
                return Some(transaction);
            }
            let logs = receipt.log.unwrap_or_default();
            // TRC20 transfers
            if value.contract_type == TRIGGER_SMART_CONTRACT
                && logs.len() == 1
                && logs.first()?.topics.clone().unwrap_or_default().len() == 3
                && logs.first()?.topics.clone().unwrap_or_default().first()? == "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            {
                let log = logs.first()?;
                let from_string = format!("41{}", log.topics.clone().unwrap_or_default()[1].clone().chars().skip(24).collect::<String>());
                let to_string = format!("41{}", log.topics.clone().unwrap_or_default()[2].clone().chars().skip(24).collect::<String>());
                let token_id = TronAddress::from_hex(value.parameter.value.contract_address?.as_str()).unwrap_or_default();
                let from = TronAddress::from_hex(from_string.as_str()).unwrap_or_default();
                let to = TronAddress::from_hex(to_string.as_str()).unwrap_or_default();
                let value = BigUint::from_str_radix(&log.data.clone().unwrap_or_default(), 16).unwrap();
                let asset_id = AssetId {
                    chain,
                    token_id: Some(token_id),
                };

                let transaction = Transaction::new(
                    transaction.tx_id,
                    asset_id,
                    from,
                    to,
                    None,
                    TransactionType::Transfer,
                    state,
                    fee,
                    chain.as_asset_id(),
                    value.to_string(),
                    None,
                    None,
                    created_at,
                );

                return Some(transaction);
            }
        }
        None
    }

    pub fn map_validators(witnesses: WitnessesList) -> Vec<StakeValidator> {
        witnesses.witnesses.into_iter().map(|x| StakeValidator::new(x.address, x.url)).collect()
    }
}
