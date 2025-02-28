use crate::block_explorer::{BlockExplorer, Metadata};

static BLOCKVISION_NAME: &str = "BlockVision";

pub struct BlockVision {
    pub meta: Metadata,
}

impl BlockVision {
    pub fn new_monad() -> Box<Self> {
        Box::new(Self {
            meta: Metadata {
                name: BLOCKVISION_NAME,
                base_url: "https://monadexplorer.com", // FIXME verify
            },
        })
    }
}

impl BlockExplorer for BlockVision {
    fn name(&self) -> String {
        self.meta.name.into()
    }

    fn get_tx_url(&self, hash: &str) -> String {
        format!("{}/tx/{}", self.meta.base_url, hash)
    }

    fn get_address_url(&self, address: &str) -> String {
        format!("{}/address/{}", self.meta.base_url, address)
    }

    fn get_token_url(&self, token: &str) -> Option<String> {
        format!("{}/token/{}", self.meta.base_url, token).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_monad_tx_url() {
        let explorer = BlockVision::new_monad();
        assert_eq!(
            explorer.get_tx_url("0xffeb2e6b3ef054fea3f5c0320c9ae3ec4e417ca58b97c4a26667414b0e423782"),
            "https://monadexplorer.com/tx/0xffeb2e6b3ef054fea3f5c0320c9ae3ec4e417ca58b97c4a26667414b0e423782"
        );
    }
}
