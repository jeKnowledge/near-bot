use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::{self, json},
    setup_alloc, AccountId,
};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    inside: LookupMap<AccountId, bool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            inside: LookupMap::new(b"s".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    // NFT CONTRACT tests.jeknowledge.testnet.nft_metadata

    //TODO: fazer crosscontract call para tests.jeknowledge.testnet e verificar se uma conta tem um nft
    pub fn has_nft(&self, account_id: AccountId) -> bool {

        //cross contract call to tests.jeknowledge.testnet
    }
}
