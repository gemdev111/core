use std::cmp::min;

use primitives::EVMChain;

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct EVMChainConfig {
    pub min_priority_fee: u64,
    pub is_opstack: bool,
    pub rewards_percentiles: EVMHistoryRewardPercentiles,
    pub fee_history_blocks: u64,
}

#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct EVMHistoryRewardPercentiles {
    pub slow: u64,
    pub normal: u64,
    pub fast: u64,
}

pub fn get_evm_chain_config(chain: EVMChain) -> EVMChainConfig {
    let block_time = chain.to_chain().block_time();
    // max 15 blocks
    let blocks = min(60 * 1000 / block_time, 15);

    EVMChainConfig {
        min_priority_fee: chain.min_priority_fee(),
        is_opstack: chain.is_opstack(),
        rewards_percentiles: EVMHistoryRewardPercentiles {
            slow: 20,
            normal: 40,
            fast: 60,
        },
        fee_history_blocks: blocks as u64,
    }
}
