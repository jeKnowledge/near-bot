use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::{self, json},
    setup_alloc, AccountId, PromiseError,
};

setup_alloc!();

const NFT_CONTRACT: &str = "test.jeknowledge.testnet";

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub nft_account: AccountId,
    inside: LookupMap<AccountId, bool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            nft_account: AccountId(NFT_CONTRACT.to_string()),
            inside: LookupMap::new(b"s".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    // NFT CONTRACT tests.jeknowledge.testnet.nft_metadata

    pub fn check_has_nft(&self, account_id: AccountId) -> bool {
        // cross contract call to tests.jeknowledge.testnet in the function check_has_nft with the argument account_id
        let promise = nft_acc::ext(self.nft_account.clone())
            .with_static_gas(GAS(5 * TGAS))
            .check_has_nft(account_id.clone());

        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS(5 * TGAS))
                .check_has_nft_callback(account_id.clone()),
        );
    }

    #[private]
    pub fn check_has_nft_callback(
        &self,
        #[callback_result] call_result: Result<String, PromiseError>,
    ) -> String {
        if call_result.is_err() {
            log!("Error: {:?}", call_result);
            return "Error".to_string();
        }

        let result: String = call_result.unwrap();
        log!("result: {}", result);
        result
    }

    pub fn update_inside(&mut self, account_id: AccountId, inside: bool) -> bool {
        // check if account id has an nft
        let has_nft = self.check_has_nft(account_id.clone());

        if has_nft {
            return self.inside.insert(&account_id, &inside);
        } else {
            log!("Account {} does not have an nft", account_id);
        }
    }

   
}
