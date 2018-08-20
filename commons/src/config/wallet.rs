use serde::Deserialize;
use serde::Serialize;
use solana_sdk::signature::Keypair;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletConfig {
    pub keypair: Vec<u8>,
}

impl WalletConfig {
    // GETTERS ----------------------------------------------------------------

    pub fn keypair(&self) -> Keypair {
        Keypair::from_bytes(&self.keypair).unwrap()
    }
}
