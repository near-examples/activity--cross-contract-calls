const Runtime = require("near-sdk-as/vm/dist").Runtime;
const assert = require("assert");

let runtime = new Runtime();

let alice = runtime.newAccount("alice");
runtime.newAccount("sentences", "out/sentences.wasm");
runtime.newAccount("words.examples", "out/words.wasm")

assert(alice.call_other("sentences", "reverseWordOne").return_data.text == "elpmas",
       "text should be reversed");

assert(alice.call_other("sentences", "reverseWordTwo"))

// TODO: this still breaks, so for now no passing to call back method.
// alice.call_other("sentences", "reverseWordThree");