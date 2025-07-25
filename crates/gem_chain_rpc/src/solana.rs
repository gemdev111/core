use async_trait::async_trait;
use std::error::Error;

use crate::{ChainAssetsProvider, ChainBlockProvider, ChainStakeProvider, ChainTokenDataProvider, ChainTransactionsProvider};

use gem_solana::{
    model::ResultTokenInfo,
    rpc::{client::SolanaClient, mapper::SolanaMapper, MISSING_BLOCKS_ERRORS},
    TOKEN_PROGRAM,
};
use primitives::{chain::Chain, Asset, AssetBalance, AssetId, StakeValidator, Transaction};

pub struct SolanaProvider {
    client: SolanaClient,
}

impl SolanaProvider {
    pub fn new(client: SolanaClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ChainBlockProvider for SolanaProvider {
    fn get_chain(&self) -> Chain {
        Chain::Solana
    }

    async fn get_latest_block(&self) -> Result<i64, Box<dyn Error + Send + Sync>> {
        self.client.get_slot().await
    }

    async fn get_transactions(&self, block_number: i64) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        let block = self.client.get_block(block_number, Some("json"), Some("full"), Some(false), Some(0)).await;
        match block {
            Ok(block) => Ok(SolanaMapper::map_block_transactions(&block)),
            Err(err) => {
                if MISSING_BLOCKS_ERRORS.contains(&err.code) {
                    Ok(vec![])
                } else {
                    Err(Box::new(err))
                }
            }
        }
    }
}

#[async_trait]
impl ChainAssetsProvider for SolanaProvider {
    async fn get_assets_balances(&self, address: String) -> Result<Vec<AssetBalance>, Box<dyn Error + Send + Sync>> {
        let accounts = self.client.get_token_accounts_by_owner(&address, TOKEN_PROGRAM).await?.value;

        Ok(accounts
            .into_iter()
            .map(|x| {
                AssetBalance::new(
                    AssetId::from_token(self.get_chain(), &x.account.data.parsed.info.mint),
                    x.account.data.parsed.info.token_amount.amount.to_string(),
                )
            })
            .collect())
    }
}

#[async_trait]
impl ChainTokenDataProvider for SolanaProvider {
    async fn get_token_data(&self, token_id: String) -> Result<Asset, Box<dyn Error + Send + Sync>> {
        let token_info = self.client.get_account_info::<ResultTokenInfo>(&token_id, "jsonParsed").await?.info();
        match token_info.extensions {
            Some(_) => SolanaMapper::map_token_data_spl_token_2022(self.get_chain(), token_id, &token_info),
            None => {
                let meta = self.client.get_metaplex_data(&token_id).await?;
                SolanaMapper::map_token_data(self.get_chain(), token_id, &token_info, &meta)
            }
        }
    }
}

#[async_trait]
impl ChainTransactionsProvider for SolanaProvider {
    async fn get_transactions_by_address(&self, address: String) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        let signatures = self.client.get_signatures_for_address(&address, 15).await?;
        if signatures.is_empty() {
            return Ok(vec![]);
        }
        let transaction_ids = signatures.clone().into_iter().map(|x| x.signature).collect();
        let transactions = self.client.get_transactions(transaction_ids).await?;

        Ok(SolanaMapper::map_signatures_transactions(transactions, signatures))
    }
}

#[async_trait]
impl ChainStakeProvider for SolanaProvider {
    async fn get_validators(&self) -> Result<Vec<StakeValidator>, Box<dyn Error + Send + Sync>> {
        let vote_accounts = self.client.get_vote_accounts().await?;
        let validator_configs = self.client.get_validator_configs().await?;

        // Create a map of vote pubkey to validator name
        let mut validator_names = std::collections::HashMap::new();
        for config in validator_configs {
            validator_names.insert(config.pubkey.clone(), config.account.data.parsed.info.name.clone());
        }

        Ok(vote_accounts
            .current
            .into_iter()
            .map(|v| StakeValidator {
                id: v.vote_pubkey.clone(),
                name: validator_names.get(&v.vote_pubkey).cloned().unwrap_or(v.node_pubkey),
            })
            .collect())
    }

    async fn get_staking_apy(&self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        let inflation_rate = self.client.get_inflation_rate().await?;
        let epoch_info = self.client.get_epoch_info().await?;

        let annual_rate = inflation_rate.validator;
        let slots_in_epoch = epoch_info.slots_in_epoch as f64;
        let slot_duration_sec = 0.4;
        let epoch_days = (slots_in_epoch * slot_duration_sec) / 86400.0;
        let epochs_per_year = 365.0 / epoch_days;

        let per_epoch = annual_rate / epochs_per_year;
        let apy = (1.0 + per_epoch).powf(epochs_per_year) - 1.0;

        Ok(apy * 100.0)
    }
}
