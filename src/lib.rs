use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};

// https://wallet.testnet.near.org/login/?referrer=NEAR+CLI&public_key=ed25519%3AFwoSYxPFYgKBmRiFZkbkb9nwYJLjVrTULWKek8CRBff3&success_url=http%3A%2F%2F127.0.0.1%3A5000

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u64, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl Counter {    
    pub fn increment(&mut self) {
        self.val += 1;
    }

    pub fn decrement(&mut self) {
        self.val += 1;
    }

    pub fn get_num(&self) -> u64 {
        self.val
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ERC721 {
    owners: HashMap<u64, AccountId>,
    token_uris: HashMap<u64, String>,
    token_ids: Counter,
    balances: HashMap<u64, u64>,
    token_approvals: HashMap<u64, AccountId>,
    operator_approvals: HashMap<AccountId, HashMap<AccountId, bool>>
}

#[near_bindgen]
impl ERC721 {

    pub fn mint(&mut self, recpient: AccountId, uri: String) -> u64 {
        self.token_ids.increment();
        let num = self.token_ids.get_num();
        self._mint(recpient, num);
        self._set_token_uri(num, uri);
        num
    }

    pub fn transfer(&mut self, to: AccountId) {

    }

    fn _set_token_uri(&mut self, num: u64, uri: String) {
        self.token_uris.insert(num, uri);
    }

    fn _mint(&mut self, og_address: AccountId, item_id: u64) {
        self.owners.insert(item_id, og_address);
    }


    fn _transfer(&mut self, to: AccountId, from: AccountId, token_id: u64) {

    }

}

#[test]
fn counter_works() {
    let mut contract = Counter{ val: 0 };
    contract.increment();
    // confirm that we received 1 when calling get_num
    println!("Value after increment: {}", contract.get_num());
    assert_eq!(1, contract.get_num());
}
