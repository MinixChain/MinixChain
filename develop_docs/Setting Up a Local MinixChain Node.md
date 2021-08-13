# Setting Up a MinixChain Node and Connecting to the Polkadot JS

## Introduction

This guide outlines the steps needed to create a development node for testing the Ethereum compatibility features of MinixChain.

A MinixChain development node is your own personal development environment for building and testing applications on MinixChain. For Ethereum developers, it is comparable to Ganache. It enables you to get started quickly and easily without the overhead of a relay chain. 



## Getting Started

First, start by cloning  the MinixChain repo that you can find here:

https://github.com/MinixChain/

```
git clone -b dev https://github.com/MinixChain/MinixChain.git
cd MinixChain
```

Next, install Substrate and all its prerequisites (including Rust) by executing:

```
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

Once you have followed all of the procedures above, it's time to build the development node by running:

```
cargo build --release
```

Then, you will want to run the node using the following command:

```
./target/release/minix --tmp --chain=dev-evm --alice --rpc-port=8545 --rpc-cors=all -levm=trace
```

## Connecting Polkadot JS Apps to a Local MinixChain Node

The development node is a Substrate-based node, so you can interact with it using standard Substrate tools. The two provided RPC endpoints are:

- HTTP: `http://127.0.0.1:8545`
- WS: `ws://127.0.0.1:9944`

Start by connecting to it with Polkadot JS Apps. Open a browser to: https://polkadot.js.org/apps/#/explorer. This will open Polkadot JS Apps, which automatically connects to Polkadot MainNet.

![image-20210804155318675](image-20210804155318675.png)

Click on the top left corner to open the menu to configure the networks, and then navigate down to open the Development sub-menu. In there, you will want to toggle the "Local Node" option, which points Polkadot JS Apps to `ws://127.0.0.1:9944`. Next, select the Switch button, and the site should connect to your MinixChain development node.

![image-20210804155431152](image-20210804155431152.png)



With Polkadot JS Apps connected, you will see some information similar to the following:

![image-20210804155957328](image-20210804155957328.png)