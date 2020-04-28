const Runtime = require("near-sdk-as/vm/dist").Runtime;
const assert = require("assert");

let runtime = new Runtime();

let alice = runtime.newAccount("alice");
runtime.newAccount("rust", "rust/res/rust_example.wasm");

let res  = (alice.call_other("rust", "reverse", {word: {text: "hello world"}}))
console.log(JSON.stringify(res, null , 2));
// runtime.newAccount("words.examples", "out/words.wasm")

// assert(alice.call_other("sentences", "reverseWordOne").return_data.text == "elpmas",
//        "text should be reversed");

// assert(alice.call_other("sentences", "reverseWordTwo"))

// TODO: this still breaks, so for now no passing to call back method.
// alice.call_other("sentences", "reverseWordThree");

