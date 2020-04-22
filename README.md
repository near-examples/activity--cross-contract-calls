![Near, Inc. logo](https://nearprotocol.com/wp-content/themes/near-19/assets/img/logo.svg?t=1553011311)

# Template for NEAR Protocol workshop activities

## Environment Setup

##### IMPORTANT: Make sure you have the latest version of NEAR Shell and Node Version >= 12.x 

1. [Node.js](https://nodejs.org/en/download/package-manager/)
2. (optional) `near-shell`

```
npm i -g near-shell
```

3. (optional) yarn

```
npm i -g yarn
```

4. Clone this repo locally

5. Run `yarn` in the repo folder to install dependencies

## Working with contracts

### To run unit tests

```bash
yarn test
```

### To build the contract

```bash
yarn build
```

### To deploy the contract


1. Login with NEAR Shell

- *You will need to install NEAR Shell first if you haven't already done so*

```bash
near login
```

2. Deploy the contract to the account with which you logged in above

```bash
near deploy --accountId <contract account>

# for example: 
# near deploy --accountId alice
```

### To invoke methods on a deployed contract

- *Signer account may be the same as contract account for testing but will almost certainly **not be the same** in production*
- *See `assembly/main.ts` for available contract methods*

```bash
near call <contract account> <contract method> --accountId <signer account>

# for example: 
# near call alice sayMyName --accountId alice
```

