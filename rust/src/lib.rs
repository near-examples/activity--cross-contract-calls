use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use serde::{Deserialize, Serialize};
// use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Prepaid gas for making a single simple call.
// const SINGLE_CALL_GAS: u64 = 200000000000000;

// #[near_bindgen]
// #[derive(Default, BorshDeserialize, BorshSerialize)]
// pub struct CrossContract {}

#[near_bindgen]
#[derive(Default, Deserialize, Serialize)]
pub struct Word {
    text: String
}
#[near_bindgen]
impl Word {
    pub fn reverse(word: Word)-> Word {
        let old_text = word;//Word {text: String::from("hello world")};
        let text = old_text.text.chars().rev().collect::<String>();
        Word {
            text
        }
    }
}
