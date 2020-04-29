use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise, PromiseResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Prepaid gas for making a single simple call.
// const SINGLE_CALL_GAS: u64 = 200000000000000;

// #[near_bindgen]
// #[derive(Default, BorshDeserialize, BorshSerialize)]
// pub struct CrossContract {}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub struct Word {
    text: String,
}

#[near_bindgen]
impl Word {
    pub fn reverse(word: Word) -> Word {
        env::log(format!("reversing word [{}]", word.text).as_bytes());
        let old_text = word; //Word {text: String::from("hello world")};
        let text = old_text.text.chars().rev().collect::<String>();
        Word { text }
    }

    pub fn reverse_three() {
        env::log(format!("calling reverse_three").as_bytes());
        let account_id = env::current_account_id();
        let word = Word {
            text: String::from("hello again 2"),
        };

        let promise = Promise::new(account_id.clone()).function_call(
            b"reverse".to_vec(),
            json!({ "word": word }).to_string().as_bytes().to_vec(),
            0,
            0,
        );

        let promise1 = Promise::new(account_id.clone());

        let drow = Word {
            text: String::from("2 niaga olleh"),
        };

        promise
            .then(promise1)
            .function_call(
                b"reverse_three_callback".to_vec(),
                json!({ "word": drow }).to_string().as_bytes().to_vec(),
                0,
                0,
            )
            .as_return();
    }

    pub fn reverse_three_callback(word: Word) -> bool {
        env::log(format!("calling reverse_three_callback").as_bytes());

        let result: Word = match env::promise_result(0) {
            PromiseResult::Successful(x) => serde_json::from_slice(&x).unwrap(),
            _ => return false,
        };

        env::log(format!("result.text is [{}]", result.text).as_bytes());
        env::log(format!("word.text is [{}]", word.text).as_bytes());
        env::log(format!("result.text == word.text is [{}]", result.text == word.text).as_bytes());

        result.text == word.text
    }

    pub fn reverse_two() {
        env::log(format!("calling reverse_two").as_bytes());
        let account_id = env::current_account_id();
        let word = Word {
            text: String::from("hello again 2"),
        };

        let promise = Promise::new(account_id.clone()).function_call(
            b"reverse".to_vec(),
            json!({ "word": word }).to_string().as_bytes().to_vec(),
            0,
            0,
        );

        let promise1 = Promise::new(account_id.clone());

        promise
            .then(promise1)
            .function_call(
                b"reverse_two_callback".to_vec(),
                json!({}).to_string().as_bytes().to_vec(),
                0,
                0,
            )
            .as_return();
    }

    pub fn reverse_two_callback() -> bool {
        env::log(format!("calling reverse_two_callback").as_bytes());

        let word = Word {
            text: String::from("2 niaga olleh"),
        };

        let result: Word = match env::promise_result(0) {
            PromiseResult::Successful(x) => serde_json::from_slice(&x).unwrap(),
            _ => return false,
        };

        env::log(format!("result.text is [{}]", result.text).as_bytes());
        env::log(format!("word.text is [{}]", word.text).as_bytes());
        env::log(format!("result.text == word.text is [{}]", result.text == word.text).as_bytes());

        result.text == word.text
    }

    pub fn reverse_one() {
        let account_id = env::current_account_id();
        let word = Word {
            text: String::from("hello again"),
        };

        Promise::new(account_id)
            .function_call(
                b"reverse".to_vec(),
                json!({ "word": word }).to_string().as_bytes().to_vec(),
                0,
                0,
            )
            .as_return();
    }
}
