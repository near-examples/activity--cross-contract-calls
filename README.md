<br />
<br />

<p>
<img src="https://nearprotocol.com/wp-content/themes/near-19/assets/img/logo.svg?t=1553011311" width="240">
</p>

<br />
<br />

## Template for NEAR Protocol workshop activities

### Requirements

##### IMPORTANT: Make sure you have the latest version of NEAR Shell and Node Version > 12.x 

1. [Node.js](https://nodejs.org/en/download/package-manager/)
2. (optional) near-shell

```
npm i -g near-shell
```
3. (optional) yarn
```
npm i -g yarn
```

### Working with contracts

#### To run unit tests

```bash
yarn test
```

#### To build the contract

```bash
yarn build
```

#### To deploy the contract

1. Login with NEAR Shell

*You will need to install NEAR Shell first if you haven't already done so*

```bash
near login
```

2. Deploy the contract to the account with which you logged in above

```bash
yarn deploy --accountId <your account id>
```
