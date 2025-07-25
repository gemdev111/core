use chrono::DateTime;
use primitives::{chain::Chain, Transaction, TransactionState, TransactionType};

use super::model::{Action, BlockHeader};

pub struct NearMapper;

impl NearMapper {
    pub fn map_transaction(chain: Chain, header: BlockHeader, transaction: super::model::Transaction) -> Option<Transaction> {
        if transaction.actions.len() == 1 || transaction.actions.len() == 2 {
            let created_at = DateTime::from_timestamp_nanos(header.timestamp as i64);

            match &transaction.actions.last()? {
                Action::Transfer { deposit } => {
                    let asset_id = chain.as_asset_id();
                    let transaction = Transaction::new(
                        transaction.hash,
                        asset_id.clone(),
                        transaction.signer_id,
                        transaction.receiver_id,
                        None,
                        TransactionType::Transfer,
                        TransactionState::Confirmed,
                        "830000000000000000000".to_string(), // Standard Near transaction fee
                        asset_id,
                        deposit.clone(),
                        None,
                        None,
                        created_at,
                    );
                    return Some(transaction);
                }
                Action::CreateAccount | Action::Other(_) => return None,
            }
        }
        None
    }
}
