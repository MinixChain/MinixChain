# Interacting with MinixChain Using Truffle

## Introduction

This guide walks through the process of deploying a Solidity-based smart contract to a MinixChain node using [Truffle](https://www.trufflesuite.com/), a commonly used development tool for smart contracts on Ethereum. Given MinixChain Ethereum compatibility features, Truffle can be used directly with a MinixChain node.

## Checking Prerequisites

We need to install Node.js (we'll use v15.x) and the npm package manager. You can download directly from [Node.js](https://nodejs.org/en/download/) or in your terminal:

Ubuntu

```
curl -sL https://deb.nodesource.com/setup_15.x | sudo -E bash -

sudo apt install -y nodejs
```

MacOS

```
# You can use homebrew (https://docs.brew.sh/Installation)
brew install node

# Or you can use nvm (https://github.com/nvm-sh/nvm)
nvm install node
```



## Running a Development Node

To set up a MinixChain development node, you can follow tutorial of  *Setting Up a Local MinixChain Node*.

## Deploying a Contract to MinixChain Using Truffle

To use most Truffle commands, you need to run them against an existing Truffle project. So the first step is to create a Truffle project.

You can create a bare project template, but for those just getting started, you can use [Truffle Boxes](https://www.trufflesuite.com/boxes), which are example applications and project templates. We'll use the [MetaCoin box](https://www.trufflesuite.com/boxes/metacoin), which creates a token that can be transferred between accounts.

Let's start,  follow the steps below.

1.Create a new directory for your Truffle project:

`mkdir MetaCoin`

`cd MetaCoin`

2.Download ("unbox") the MetaCoin box:

`truffle unbox metacoin`

Once this operation is completed, you'll now have a project structure with the following items:

**contracts/**: Directory for Solidity contracts
**migrations/**: Directory for scriptable deployment files
**test/**: Directory for test files for testing your application and contracts
**truffle-config.js**: Truffle configuration file



3.Configure the network in the truffle-config.js file :

```
module.exports = {

  networks: {
    development: {
      host: "127.0.0.1",
      port: 8545,
      network_id: "*"
    }
  }    

};
```



4.Compile the Truffle project

To compile a Truffle project, change to the root of the directory where the project is located and then type the following into a terminal:

```
/MetaCoin$ truffle compile

Compiling your contracts...
===========================
> Compiling ./contracts/ConvertLib.sol
> Compiling ./contracts/MetaCoin.sol
> Compiling ./contracts/Migrations.sol
> Artifacts written to /home/test/TruffleTest/MetaCoin/build/contracts
> Compiled successfully using:
   - solc: 0.5.16+commit.9c3226ce.Emscripten.clang
   
```

5.Deploy the Truffle project

To deploy the  Truffle project, execute `truffle migrate`:

```
/MetaCoin$ truffle migrate

Compiling your contracts...
===========================
> Everything is up to date, there is nothing to compile.



Starting migrations...
======================
> Network name:    'development'
> Network id:      1627899638136
> Block gas limit: 6721975 (0x6691b7)


1_initial_migration.js
======================

   Deploying 'Migrations'
   ----------------------
   > transaction hash:    0x80a58c383b64d025270122264090a5adddd28cc6ebea4093e09408a4b9a89943
   > Blocks: 0            Seconds: 0
   > contract address:    0xD156AF6dc2C85635598de526709188D110650E76
   > block number:        1
   > block timestamp:     1627899705
   > account:             0x21ca314A5105eEE8e591aA7B6b8c35D33DcF0D12
   > balance:             99.9967165
   > gas used:            164175 (0x2814f)
   > gas price:           20 gwei
   > value sent:          0 ETH
   > total cost:          0.0032835 ETH


   > Saving migration to chain.
   > Saving artifacts
   -------------------------------------
   > Total cost:           0.0032835 ETH


2_deploy_contracts.js
=====================

   Deploying 'ConvertLib'
   ----------------------
   > transaction hash:    0xba51d392a0ef16f319f9e2ae040d7f200063f63074da4f8065bbee9130be895d
   > Blocks: 0            Seconds: 0
   > contract address:    0x4c5762ddE9f44431E485e473e0AACBd79d1B837d
   > block number:        3
   > block timestamp:     1627899705
   > account:             0x21ca314A5105eEE8e591aA7B6b8c35D33DcF0D12
   > balance:             99.99396028
   > gas used:            95470 (0x174ee)
   > gas price:           20 gwei
   > value sent:          0 ETH
   > total cost:          0.0019094 ETH


   Linking
   -------
   * Contract: MetaCoin <--> Library: ConvertLib (at address: 0x4c5762ddE9f44431E485e473e0AACBd79d1B837d)

   Deploying 'MetaCoin'
   --------------------
   > transaction hash:    0xc51e87889a78acefd2db7fea1afe6c7714d633575c65d8b2f046e33029b21d94
   > Blocks: 0            Seconds: 0
   > contract address:    0xAC9859CbC99fB36320aED71bB707c4563Ff58D0D
   > block number:        4
   > block timestamp:     1627899705
   > account:             0x21ca314A5105eEE8e591aA7B6b8c35D33DcF0D12
   > balance:             99.98822898
   > gas used:            286565 (0x45f65)
   > gas price:           20 gwei
   > value sent:          0 ETH
   > total cost:          0.0057313 ETH


   > Saving migration to chain.
   > Saving artifacts
   -------------------------------------
   > Total cost:           0.0076407 ETH


Summary
=======
> Total deployments:   3
> Final cost:          0.0109242 ETH
   
```



6.Interacting with your contracts

Contract abstractions are the bread and butter of interacting with Ethereum contracts from Javascript. In short, contract abstractions are wrapper code that makes interaction with your contracts easy, in a way that lets you forget about the many engines and gears executing under the hood. Truffle uses its own contract abstraction via the [@truffle/contract](https://github.com/trufflesuite/truffle/tree/master/packages/contract) module, and it is this contract abstraction that's described below.

```solidity
pragma solidity >=0.4.25 <0.6.0;

import "./ConvertLib.sol";

// This is just a simple example of a coin-like contract.
// It is not standards compatible and cannot be expected to talk to other
// coin/token contracts. If you want to create a standards-compliant
// token, see: https://github.com/ConsenSys/Tokens. Cheers!

contract MetaCoin {
    mapping (address => uint) balances;

    event Transfer(address indexed _from, address indexed _to, uint256 _value);

    constructor() public {
        balances[tx.origin] = 10000;
    }

    function sendCoin(address receiver, uint amount) public returns(bool sufficient) {
        if (balances[msg.sender] < amount) return false;
        balances[msg.sender] -= amount;
        balances[receiver] += amount;
        emit Transfer(msg.sender, receiver, amount);
        return true;
    }

    function getBalanceInEth(address addr) public view returns(uint){
        return ConvertLib.convert(getBalance(addr),2);
    }

    function getBalance(address addr) public view returns(uint) {
        return balances[addr];
    }
}
```

Now let's look at the Javascript object called `MetaCoin` provided for us by Truffle, as made available in the [Truffle console](https://www.trufflesuite.com/docs/truffle/getting-started/using-truffle-develop-and-the-console):

```
truffle(development)> let instance = await MetaCoin.deployed()
truffle(development)> instance


   ...

   methods: {
      sendCoin: [Function: bound _createTxObject],
      '0x90b98a11': [Function: bound _createTxObject],
      'sendCoin(address,uint256)': [Function: bound _createTxObject],
      getBalanceInEth: [Function: bound _createTxObject],
      '0x7bd703e8': [Function: bound _createTxObject],
      'getBalanceInEth(address)': [Function: bound _createTxObject],
      getBalance: [Function: bound _createTxObject],
      '0xf8b2cb4f': [Function: bound _createTxObject],
      'getBalance(address)': [Function: bound _createTxObject]
    },
    events: {
      Transfer: [Function: bound ],
      '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef': [Function: bound ],
      'Transfer(address,address,uint256)': [Function: bound ],
      allEvents: [Function: bound ]
    },
    _address: '0xAC9859CbC99fB36320aED71bB707c4563Ff58D0D',
    
    
 ...


   
```

Making a transaction:

There are three functions on the MetaCoin contract that we can execute. If you analyze each of them, you'll see that `sendCoin` is the only function that aims to make changes to the network. The goal of `sendCoin` is to "send" some Meta coins from one account to the next, and these changes should persist.

When calling `sendCoin`, we'll execute it as a transaction. In the following example, we'll send 10 Meta coin from one account to another, in a way that persists changes on the network:

```
      
truffle(development)> let accounts = await web3.eth.getAccounts()
undefined
truffle(development)> instance.sendCoin(accounts[1], 10, {from: accounts[0]})
{
  tx: '0x9c434c333be966cbfc39cb898637abd05728c605574bafdbd102124941097428',
  receipt: {
    transactionHash: '0x9c434c333be966cbfc39cb898637abd05728c605574bafdbd102124941097428',
    transactionIndex: 0,
    blockHash: '0xe6662f4600baac7225853c9cabddc6064aec431ef87ed4b09a0bc4c6f29d43c6',
    blockNumber: 6,
    from: '0x21ca314a5105eee8e591aa7b6b8c35d33dcf0d12',
    to: '0xac9859cbc99fb36320aed71bb707c4563ff58d0d',
    gasUsed: 51508,
    cumulativeGasUsed: 51508,
    contractAddress: null,
    logs: [ [Object] ],
    status: true,
    logsBloom: '0x00000000000000000004000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000010000000000000000000000000000000000000000000000010000000000000000000000080000000000000000000000000000000000000000800010000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000040000000000000',
    rawLogs: [ [Object] ]
  },
  logs: [
    {
      logIndex: 0,
      transactionIndex: 0,
      transactionHash: '0x9c434c333be966cbfc39cb898637abd05728c605574bafdbd102124941097428',
      blockHash: '0xe6662f4600baac7225853c9cabddc6064aec431ef87ed4b09a0bc4c6f29d43c6',
      blockNumber: 6,
      address: '0xAC9859CbC99fB36320aED71bB707c4563Ff58D0D',
      type: 'mined',
      removed: false,
      id: 'log_d631b672',
      event: 'Transfer',
      args: [Result]
    }
  ]
}


   
```

Making a call:

Continuing with MetaCoin, notice the `getBalance` function is a great candidate for reading data from the network. It doesn't need to make any changes, as it just returns the MetaCoin balance of the address passed to it. Let's give it a shot:

```
    
truffle(development)> let balance = await instance.getBalance(accounts[0])
undefined
truffle(development)>  balance.toNumber()
9990


   
```

