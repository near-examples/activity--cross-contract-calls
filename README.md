![Near, Inc. logo](https://nearprotocol.com/wp-content/themes/near-19/assets/img/logo.svg?t=1553011311)

# Exploring Cross Contract Calls in AssemblyScript

## Working with contracts

### To run unit tests

```bash
yarn test
```

### To build the contracts

- *Two contracts will be built*

```bash
yarn build
```

### To invoke methods on a deployed contract

#### `reverseWordOne`

```bash
yarn near-vm --wasm-file ./out/sentences.wasm --method-name reverseWordOne
```

```json
{
  "outcome": {
    "balance": "10000000000000000000000000",
    "storage_usage": 100,
    "return_data": {
      "ReceiptIndex": 0
    },
    "burnt_gas": 2378724083956,
    "used_gas": 4670329121700,
    "logs": []
  },
  "err": null,
  "receipts": [
    {
      "receipt_indices": [],
      "receiver_id": "words.examples",
      "actions": [
        {
          "FunctionCall": {
            "method_name": "reverse",
            "args": "{\"word\":{\"text\":\"sample\",\"lang\":\"en-us\"}}",
            "gas": 0,
            "deposit": 0
          }
        }
      ]
    }
  ],
  "state": {}
}
```

#### `reverseWordTwo`

```bash
yarn near-vm --wasm-file ./out/sentences.wasm --method-name reverseWordTwo
```

**error:** `Bad error case! Output is non-deterministic TypeId { t: 7549865886324542212 } "unknown error"`

```json
{
  "outcome": {
    "balance": "10000000000000000000000000",
    "storage_usage": 100,
    "return_data": "None",
    "burnt_gas": 2457198454953,
    "used_gas": 4748803492697,
    "logs": [
      "alice"
    ]
  },
  "err": {
    "FunctionCallError": {
      "WasmTrap": {
        "msg": "unknown"
      }
    }
  },
  "receipts": [
    {
      "receipt_indices": [],
      "receiver_id": "words.examples",
      "actions": [
        {
          "FunctionCall": {
            "method_name": "reverse",
            "args": "{\"word\":{\"text\":\"sample\",\"lang\":\"en-us\"}}",
            "gas": 0,
            "deposit": 0
          }
        }
      ]
    }
  ],
  "state": {}
}
```

