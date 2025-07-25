use chrono::DateTime;
use primitives::{chain::Chain, Transaction, TransactionState, TransactionType};
use tonlib_core::TonAddress;

use super::model::Transaction as TonTransaction;

pub struct TonMapper;

impl TonMapper {
    pub fn parse_address(address: &str) -> Option<String> {
        Some(TonAddress::from_hex_str(address).ok()?.to_base64_url())
    }

    pub fn map_transactions(chain: Chain, transactions: Vec<TonTransaction>) -> Vec<Transaction> {
        transactions
            .into_iter()
            .flat_map(|x| Self::map_transaction(chain, x))
            .collect::<Vec<Transaction>>()
    }

    pub fn map_transaction(chain: Chain, transaction: TonTransaction) -> Option<Transaction> {
        if transaction.transaction_type == "TransOrd" && transaction.out_msgs.len() == 1 && transaction.out_msgs.first()?.op_code.is_none() {
            let asset_id = chain.as_asset_id();
            let out_message = transaction.out_msgs.first()?;
            let from = Self::parse_address(&out_message.source.address)?;
            let to: String = match &out_message.destination {
                Some(destination) => Self::parse_address(&destination.address)?,
                None => "".into(),
            };
            let value = out_message.value.to_string();
            let state = if transaction.success {
                TransactionState::Confirmed
            } else {
                TransactionState::Failed
            };
            let created_at = DateTime::from_timestamp(transaction.utime, 0)?;
            let hash = transaction.in_msg?.hash.clone();
            //TODO: Implement memo
            let memo: Option<String> = None; //out_message.decoded_body.clone().text;

            let transaction = Transaction::new(
                hash,
                asset_id.clone(),
                from,
                to,
                None,
                TransactionType::Transfer,
                state,
                transaction.total_fees.to_string(),
                asset_id,
                value,
                memo,
                None,
                created_at,
            );
            return Some(transaction);
        }
        None
    }
}
