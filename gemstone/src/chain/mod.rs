use primitives::Chain;

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct ChainConfig {
    pub network_id: String,
    pub transaction_timeout: f64,
    pub slip_44: i32,
    pub rank: i32,
    pub denom: Option<String>,
    pub default_asset_type: Option<String>,
    pub account_activation_fee: Option<i32>,
    pub account_activation_fee_url: Option<String>,
    pub is_swap_supported: bool,
    pub is_stake_supported: bool,
}

pub fn get_chain_config(chain: Chain) -> ChainConfig {
    ChainConfig {
        network_id: chain.network_id().to_string(),
        transaction_timeout: chain_transaction_timeout_seconds(chain),
        slip_44: chain.as_slip44() as i32,
        rank: chain.rank(),
        denom: chain.as_denom().map(|x| x.to_string()),
        default_asset_type: chain.default_asset_type().map(|x| x.as_ref().to_string()),
        account_activation_fee: chain.account_activation_fee(),
        account_activation_fee_url: account_activation_fee_url(chain).map(|x| x.to_string()),
        is_swap_supported: chain.is_swap_supported(),
        is_stake_supported: chain.is_stake_supported(),
    }
}

pub fn account_activation_fee_url(chain: Chain) -> Option<String> {
    match chain {
        Chain::Xrp => Some("https://xrpl.org/docs/concepts/accounts/reserves#base-reserve-and-owner-reserve".into()),
        Chain::Stellar => Some("https://developers.stellar.org/docs/learn/fundamentals/lumens#minimum-balance".into()),
        _ => None,
    }
}

fn chain_transaction_timeout_seconds(chain: Chain) -> f64 {
    match chain {
        Chain::Bitcoin => 28800_f64,
        Chain::Litecoin | Chain::Doge => 7200_f64,
        Chain::Solana => 300_f64,
        Chain::Ethereum
        | Chain::SmartChain
        | Chain::Polygon
        | Chain::Thorchain
        | Chain::Cosmos
        | Chain::Osmosis
        | Chain::Arbitrum
        | Chain::Ton
        | Chain::Tron
        | Chain::Optimism
        | Chain::Aptos
        | Chain::Base
        | Chain::AvalancheC
        | Chain::Sui
        | Chain::Xrp
        | Chain::OpBNB
        | Chain::Fantom
        | Chain::Gnosis
        | Chain::Celestia
        | Chain::Injective
        | Chain::Sei
        | Chain::Manta
        | Chain::Blast
        | Chain::Noble
        | Chain::ZkSync
        | Chain::Linea
        | Chain::Mantle
        | Chain::Celo
        | Chain::Near
        | Chain::World => 1800_f64, // 30 minutes
        Chain::Stellar => 600_f64,
    }
}
