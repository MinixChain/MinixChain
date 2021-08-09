# Using Ethereum Libraries to Deploy Smart Contracts on Moonbeam

## Introduction

This guide walks through using the Solidity compiler and three different Ethereum libraries to sign and send a transaction on Moonbeam manually. The three libraries covered by this tutorial are:

- [Web3.js](https://web3js.readthedocs.io/)

- [Ethers.js](https://docs.ethers.io/)

  

## Checking Prerequisites

The examples using both web3.js and ethers.js need you to install Node.js and NPM previously. For the web3.py, you need Python and PIP. As of the writing of this guide, the versions used were:

- Node.js v15.10.0
- NPM v7.5.3

Next, create a directory to store all of the relevant files:

```
mkdir incrementer && cd incrementer/
```

For the JavaScript libraries, first, you can create a simple `package.json` file (not required):

```
npm init --yes
```

In the directory, install the corresponding library and the Solidity compiler:

Web3.js

```
npm i web3 npm i solc@0.8.0
```

Ethers.js

```
npm i ethers npm i solc@0.8.0
```

## The Contract File

The contract used is a simple incrementer, arbitrarily named *Incrementer.sol*. The Solidity code is the following:

```
pragma solidity ^0.8.0;

contract Incrementer {
    uint256 public number;

    constructor(uint256 _initialNumber) {
        number = _initialNumber;
    }

    function increment(uint256 _value) public {
        number = number + _value;
    }

    function reset() public {
        number = 0;
    }
}
```

The `constructor` function, which runs when the contract is deployed, sets the initial value of the number variable stored on-chain (default is 0). The `increment` function adds the `_value` provided to the current number, but a transaction needs to be sent, which modifies the stored data. Lastly, the `reset` function resets the stored value to zero.

## Compiling the Contract

The only purpose of the compile file is to use the Solidity compiler to output the bytecode and interface (ABI) our contract. (they were arbitrarily named `compile.js`)

The compile file for both JavaScript libraries is the same as they share the JavaScript bindings for the Solidity compiler (same package).

Web3.js/Ethers.js

//filename: compile.js

```
const fs = require('fs');
const solc = require('solc');

// Get Path and Load Contract
const source = fs.readFileSync('Incrementer.sol', 'utf8');

// Compile Contract
const input = {
   language: 'Solidity',
   sources: {
      'Incrementer.sol': {
         content: source,
      },
   },
   settings: {
      outputSelection: {
         '*': {
            '*': ['*'],
         },
      },
   },
};
const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
const contractFile = tempFile.contracts['Incrementer.sol']['Incrementer'];

// Export Contract Data
module.exports = contractFile;
```

In the first part of the script, the contract's path is fetched, and its content read.

Next, the Solidity compiler's input object is built, and it is passed as input to the `solc.compile` function.

Lastly, extract the data of the `Incrementer` contract of the `Incrementer.sol` file, and export it so that the deployment script can use it.

## Deploying the Contract

Regardless of the library, the strategy to deploy the compiled smart contract is somewhat similar. A contract instance is created using its interface (ABI) and bytecode. From this instance, a deployment function is used to send a signed transaction that deploys the contract.

For simplicity, the deploy file is composed of two sections. In the first section ("Define Provider & Variables"), the library to use and the ABI and bytecode of the contract are imported. Also, the provider and account from (with the private key) are defined. 

The second section ("Deploy Contract") outlines the actual contract deployment part. Note that for this example, the initial value of the `number` variable was set to 5. Some of the key takeaways are discussed next.

Web3.js

//filename: deploy.js

```
const Web3 = require('web3');
const contractFile = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: 'http://localhost:8545',
};
const web3 = new Web3(providerRPC.development); //Change to correct network

// Variables
const account_from = {
   privateKey: 'YOUR-PRIVATE-KEY-HERE',
   address: 'PUBLIC-ADDRESS-OF-PK-HERE',
};
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;

/*
   -- Deploy Contract --
*/
const deploy = async () => {
   console.log(`Attempting to deploy from account ${account_from.address}`);

   // Create Contract Instance
   const incrementer = new web3.eth.Contract(abi);

   // Create Constructor Tx
   const incrementerTx = incrementer.deploy({
      data: bytecode,
      arguments: [5],
   });

   // Sign Transacation and Send
   const createTransaction = await web3.eth.accounts.signTransaction(
      {
         data: incrementerTx.encodeABI(),
         gas: await incrementerTx.estimateGas(),
      },
      account_from.privateKey
   );

   // Send Tx and Wait for Receipt
   const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log(
      `Contract deployed at address: ${createReceipt.contractAddress}`
   );
};

deploy();
```

Ethers.js

//filename: deploy.js

```
const ethers = require('ethers');
const contractFile = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: {
      name: 'minix-development',
      rpc: 'http://localhost:8545',
      chainId: 1500,
   },
};
const provider = new ethers.providers.StaticJsonRpcProvider(
   providerRPC.development.rpc,
   {
      chainId: providerRPC.development.chainId,
      name: providerRPC.development.name,
   }
); //Change to correct network

// Variables
const account_from = {
   privateKey: 'YOUR-PRIVATE-KEY-HERE',
};
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;

// Create Wallet
let wallet = new ethers.Wallet(account_from.privateKey, provider);

/*
   -- Deploy Contract --
*/
// Create Contract Instance with Signer
const incrementer = new ethers.ContractFactory(abi, bytecode, wallet);

const deploy = async () => {
   console.log(`Attempting to deploy from account: ${wallet.address}`);

   // Send Tx (Initial Value set to 5) and Wait for Receipt
   const contract = await incrementer.deploy([5]);
   await contract.deployed();

   console.log(`Contract deployed at address: ${contract.address}`);
};

deploy();
```

### Web3.js

In the first part of the script, the `web3` instance (or provider) is created using the `Web3` constructor with the provider RPC. By changing the provider RPC given to the constructor, you can choose which network you want to send the transaction to.

The private key, and the public address associated with it, are defined for signing the transaction and logging purposes. Only the private key is required. Also, the contract's bytecode and interface (ABI) are fetched from the compile's export.

In the second section, a contract instance is created by providing the ABI. Next, the `deploy` function is used, which needs the bytecode and arguments of the constructor function. This will generate the constructor transaction object.

Afterwards, the constructor transaction can be signed using the `web3.eth.accounts.signTransaction()` method. The data field corresponds to the bytecode, and the constructor input arguments are encoded together. Note that the value of gas is obtained using `estimateGas()` option inside the constructor transaction.

Lastly, the signed transaction is sent, and the contract's address is displayed in the terminal.

### Ethers.js

In the first part of the script, different networks can be specified with a name, RPC URL (required), and chain ID. The provider (similar to the `web3` instance) is created with the `ethers.providers.StaticJsonRpcProvider` method. An alternative is to use the `ethers.providers.JsonRpcProvide(providerRPC)` method, which only requires the provider RPC endpoint address. But this might created compatibility issues with individual project specifications.

The private key is defined to create a wallet instance, which also requires the provider from the previous step. The wallet instance is used to sign transactions. Also, the contract's bytecode and interface (ABI) are fetched from the compile's export.

In the second section, a contract instance is created with `ethers.ContractFactory()`, providing the ABI, bytecode, and wallet. Thus, the contract instance already has a signer. Next, the `deploy` function is used, which needs the constructor input arguments. This will send the transaction for contract deployment. To wait for a transaction receipt you can use the `deployed()` method of the contract deployment transaction.

Lastly, the contract's address is displayed in the terminal.

## Reading from the Contract (Call Methods)

Call methods are the type of interaction that don't modify the contract's storage (change variables), meaning no transaction needs to be sent.

For simplicity, the get file is composed of two sections. In the first section ("Define Provider & Variables"), the library to use and the ABI of the contract are imported. Also, the provider and the contract's address are defined. 

The second section ("Call Function") outlines the actual call to the contract. Regardless of the library, a contract instance is created (linked to the contract's address), from which the call method is queried. Some of the key takeaways are discussed next.

Web3.js

//filename: get.js

```
const Web3 = require('web3');
const { abi } = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: 'http://localhost:8545',
};
const web3 = new Web3(providerRPC.development); //Change to correct network

// Variables
const contractAddress = 'CONTRACT-ADDRESS-HERE';

/*
   -- Call Function --
*/
// Create Contract Instance
const incrementer = new web3.eth.Contract(abi, contractAddress);

const get = async () => {
   console.log(`Making a call to contract at address: ${contractAddress}`);

   // Call Contract
   const data = await incrementer.methods.number().call();

   console.log(`The current number stored is: ${data}`);
};

get();
```

Ethers.js

//filename: get.js

```
const ethers = require('ethers');
const { abi } = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: {
      name: 'minix-development',
      rpc: 'http://localhost:8545',
      chainId: 1500,
   },
};
const provider = new ethers.providers.StaticJsonRpcProvider(
   providerRPC.development.rpc,
   {
      chainId: providerRPC.development.chainId,
      name: providerRPC.development.name,
   }
); //Change to correct network

// Variables
const contractAddress = 'CONTRACT-ADDRESS-HERE';

/*
   -- Call Function --
*/
// Create Contract Instance
const incrementer = new ethers.Contract(contractAddress, abi, provider);

const get = async () => {
   console.log(`Making a call to contract at address: ${contractAddress}`);

   // Call Contract
   const data = await incrementer.number();

   console.log(`The current number stored is: ${data}`);
};

get();
```

### Web3.js 

In the first part of the script, the `web3` instance (or provider) is created using the `Web3` constructor with the provider RPC. By changing the provider RPC given to the constructor, you can choose which network you want to send the transaction to.

The contract's interface (ABI) and address are needed as well to interact with it.

In the second section, a contract instance is created with `web3.eth.Contract()` by providing the ABI and address. Next, the method to call can be queried with the `contract.methods.methodName(_input).call()` function, replacing `contract`, `methodName` and `_input` with the contract instance, function to call, and input of the function (if necessary). This promise, when resolved, will return the value requested.

Lastly, the value is displayed in the terminal.

### Ethers.js

In the first part of the script, different networks can be specified with a name, RPC URL (required), and chain ID. The provider (similar to the `web3` instance) is created with the `ethers.providers.StaticJsonRpcProvider` method. An alternative is to use the `ethers.providers.JsonRpcProvide(providerRPC)` method, which only requires the provider RPC endpoint address. But this might created compatibility issues with individual project specifications.

The contract's interface (ABI) and address are needed as well to interact with it.

In the second section, a contract instance is created with `ethers.Contract()`, providing its address, ABI, and the provider. Next, the method to call can be queried with the `contract.methodName(_input)` function, replacing `contract` `methodName`, and `_input` with the contract instance, function to call, and input of the function (if necessary). This promise, when resolved, will return the value requested.

Lastly, the value is displayed in the terminal.



## Interacting with the Contract (Send Methods)

Send methods are the type of interaction that modify the contract's storage (change variables), meaning a transaction needs to be signed and sent.

For simplicity, the increment file is composed of two sections. In the first section ("Define Provider & Variables"), the library to use and the ABI of the contract are imported. The provider, the contract's address, and the value of the `increment` function are also defined. 

The second section ("Send Function") outlines the actual function to be called with the transaction. Regardless of the library, a contract instance is created (linked to the contract's address), from which the function to be used is queried.

Web3.js

//filename: increment.js

```
const Web3 = require('web3');
const { abi } = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: 'http://localhost:8545',
};
const web3 = new Web3(providerRPC.development); //Change to correct network

// Variables
const account_from = {
   privateKey: 'YOUR-PRIVATE-KEY-HERE',
};
const contractAddress = 'CONTRACT-ADDRESS-HERE';
const _value = 3;

/*
   -- Send Function --
*/
// Create Contract Instance
const incrementer = new web3.eth.Contract(abi, contractAddress);

// Build Increment Tx
const incrementTx = incrementer.methods.increment(_value);

const increment = async () => {
   console.log(
      `Calling the increment by ${_value} function in contract at address: ${contractAddress}`
   );

   // Sign Tx with PK
   const createTransaction = await web3.eth.accounts.signTransaction(
      {
         to: contractAddress,
         data: incrementTx.encodeABI(),
         gas: await incrementTx.estimateGas(),
      },
      account_from.privateKey
   );

   // Send Tx and Wait for Receipt
   const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log(`Tx successful with hash: ${createReceipt.transactionHash}`);
};

increment();
```



Ethers.js

//filename: increment.js

```
const ethers = require('ethers');
const { abi } = require('./compile');

/*
   -- Define Provider & Variables --
*/
// Provider
const providerRPC = {
   development: {
      name: 'minix-development',
      rpc: 'http://localhost:8545',
      chainId: 1500,
   },
};
const provider = new ethers.providers.StaticJsonRpcProvider(
   providerRPC.development.rpc,
   {
      chainId: providerRPC.development.chainId,
      name: providerRPC.development.name,
   }
); //Change to correct network

// Variables
const account_from = {
   privateKey: 'YOUR-PRIVATE-KEY-HERE',
};
const contractAddress = 'CONTRACT-ADDRESS-HERE';
const _value = 3;

// Create Wallet
let wallet = new ethers.Wallet(account_from.privateKey, provider);

/*
   -- Send Function --
*/
// Create Contract Instance with Signer
const incrementer = new ethers.Contract(contractAddress, abi, wallet);
const increment = async () => {
   console.log(
      `Calling the increment by ${_value} function in contract at address: ${contractAddress}`
   );

   // Sign-Send Tx and Wait for Receipt
   const createReceipt = await incrementer.increment([_value]);
   await createReceipt.wait();

   console.log(`Tx successful with hash: ${createReceipt.hash}`);
};

increment();
```

### Web3.js

In the first part of the script, the `web3` instance (or provider) is created using the `Web3` constructor with the provider RPC. By changing the provider RPC given to the constructor, you can choose which network you want to send the transaction to.

The private key, and the public address associated with it, are defined for signing the transaction and logging purposes. Only the private key is required. Also, the contract's interface (ABI) and address are needed to interact with it. If necessary, you can define any variable required as input to the function you are going to interact with.

In the second section, a contract instance is created with `web3.eth.Contract()` by providing the ABI and address. Next, you can build the transaction object with the `contract.methods.methodName(_input)` function, replacing `contract`, `methodName` and `_input` with the contract instance, function to call, and input of the function (if necessary).

Afterwards, the transaction can be signed using the `web3.eth.accounts.signTransaction()` method. The data field corresponds to the transaction object from the previous step. Note that the value of gas is obtained using `estimateGas()` option inside the transaction object.

Lastly, the signed transaction is sent, and the transaction hash is displayed in the terminal.

### Ethers.js

In the first part of the script, different networks can be specified with a name, RPC URL (required), and chain ID. The provider (similar to the `web3` instance) is created with the `ethers.providers.StaticJsonRpcProvider` method. An alternative is to use the `ethers.providers.JsonRpcProvide(providerRPC)` method, which only requires the provider RPC endpoint address. But this might created compatibility issues with individual project specifications.

The private key is defined to create a wallet instance, which also requires the provider from the previous step. The wallet instance is used to sign transactions. Also, the contract's interface (ABI) and address are needed to interact with it. If necessary, you can define any variable required as input to the function you are going to interact with.

In the second section, a contract instance is created with `ethers.Contract()`, providing its address, ABI, and wallet. Thus, the contract instance already has a signer. Next, transaction corresponding to a specific function can be send with the `contract.methodName(_input)` function, replacing `contract`, `methodName` and `_input` with the contract instance, function to call, and input of the function (if necessary). To wait for a transaction receipt, you can use the `wait()` method of the contract deployment transaction.

Lastly, the transaction hash is displayed in the terminal.

## Running the Scripts

For this section, the code shown before was adapted to target a development node, which you can run by following  *Setting Up a Local MinixChain Node* .  Also, each transaction was sent from the pre-funded account that comes with the node:

- Private key: `99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342`
- Public address: `0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b`

(1). First, deploy the contract by running (note that the directory was renamed for each library):

Web3.js

```
node deploy.js
```

Ethers.js

```
node deploy.js
```

This will deploy the contract and return the address:

Web3.js

```
/web3js_incrementer$ node deploy.js
Attempting to deploy from account 0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b
Contract deployed at address: 0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a
```

Ethers.js

```
/etherjs_incrementer$ node deploy.js
Attempting to deploy from account: 0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b
Contract deployed at address: 0x42e2EE7Ba8975c473157634Ac2AF4098190fc741
```



(2). Next,  set the contract address in the get.js and increment.js ,  then , run the increment file. You can use the get file to verify the value of the number stored in the contract before and after increment it:

Web3.js

```
# Get value
node get.js 
# Increment value
node increment.js
# Get value
node get.js
```

Ethers.js

```
# Get value
node get.js 
# Increment value
node increment.js
# Get value
node get.js
```

This will display the value before the increment transaction, the hash of the transaction, and the value after the increment transaction:

Web3.js

```
/web3js_incrementer$ node get.js 
Making a call to contract at address: 0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a
The current number stored is: 5

/web3js_incrementer$ node increment.js
Calling the increment by 3 function in contract at address: 0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a
Tx successful with hash: 0x837a599fd07ab83d11a2d2ee097f9afc82dcce491d2acf33eb32546b15a23fd3


/web3js_incrementer$ node get.js
Making a call to contract at address: 0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a
The current number stored is: 8
```



Ethers.js

```
/etherjs_incrementer$ node get.js
Making a call to contract at address: 0x42e2EE7Ba8975c473157634Ac2AF4098190fc741
The current number stored is: 5

/etherjs_incrementer$ node increment.js 
Calling the increment by 3 function in contract at address: 0x42e2EE7Ba8975c473157634Ac2AF4098190fc741
Tx successful with hash: 0xcce5908307dc0d2e29b3dda1503cc2c4b3ea710db7d520567c5b81ea516a4136

/etherjs_incrementer$ node get.js
Making a call to contract at address: 0x42e2EE7Ba8975c473157634Ac2AF4098190fc741
The current number stored is: 8
```





