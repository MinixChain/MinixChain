# MinixChain

MiniX Chain 是一个基于 Substrate 框架,采用 aura + grandpa 共识的开放联盟链，最后会通过 Spider 跨链协议衍生为 ChainX 的平行链。MiniX Chain 不会发行代币，但为了防止链上的 DDOS 攻击, 将发行Mini积分，用作该联盟链的手续费。Mini积分将可能支持 ChainX 代币的单向兑换。

MiniX Chain 作为开放联盟链，将根据各国法律，在合法合规的前提下, 支持类似 DID/NFT 等应用的上线。Coming将成为首个用例, 将其中心化数字身份和 NFT 等应用部署到MiniX联盟链中，使用Mini积分作为手续费。

当然，作为开放联盟链，我们欢迎各大联盟链厂商公司加盟做节点，以及各大应用加入该开放联盟链生态。


Based on Substrate framework, MiniX Chain is an open Alliance Blockchain adopting aura + grandpa consensus algorithm. It will function as ChainX parachain via Spider inter-chain protocol. MiniX Chain will not issue token, but credits to prevent DDOS attacks and to pay for services. Mini credits will also be likely to be exchanged for ChainX tokens.
 
MiniX Chain as an open alliance chain will adhere to laws and regulations of various countries, which is a prerequisite for it to support the launch of applications like DID/NFT. Coming will become the first user of MiniX alliance chain onto which its decentralized identity system and NFT will be deployed with Mini credit as service fees.

Of course as an open alliance chain, we do hope to attract more alliance-chain companies as nodes and applications to enrich the ecosystem.


## 1. types
```json
{
  "Address": "MultiAddress",
  "LookupSource": "MultiAddress",
  "Cid": "u64",
  "BondType": "u16",
  "BondData": {
    "bond_type": "BondType",
    "data": "Bytes"
  },
  "CidDetails": {
    "owner": "AccountId",
    "bonds": "Vec<BondData>",
    "card":  "Bytes"
  },
  "PalletAuctionId": "[u8;4]",
  "Auction": {
    "seller": "AccountId",
    "start_price": "Balance",
    "end_price": "Balance",
    "duration": "BlockNumber",
    "start": "BlockNumber"
  },
  "OpCode": {
    "_enum": [
      "Transfer"
    ]
  }
}
```

## 2. rpc types
```json
{
  "comingId": {
    "getAccountId": {
      "description": "comingId getAccountId",
      "params": [
        {
          "name": "cid",
          "type": "Cid"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "Option<AccountId>"
    },
    "getCids": {
      "description": "comingId getCids",
      "params": [
        {
          "name": "account",
          "type": "AccountID"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "Vec<Cid>"
    },
    "getBondData": {
      "description": "comingId getBondData",
      "params": [
        {
          "name": "cid",
          "type": "Cid"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "Option<CidDetails>"
    },
    "getCard": {
      "description": "comingId getCard",
      "params": [
        {
          "name": "cid",
          "type": "Cid"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "Option<Bytes>"
    }
  },
  "comingAuction": {
    "getPrice": {
      "description": "comingAuction getPrice",
      "params": [
        {
          "name": "cid",
          "type": "Cid"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "string"
    }
  },
  "ts": {
    "computeScriptHash": {
      "description": "threshold signature computeScriptHash",
      "params": [
        {
          "name": "account",
          "type": "AccountID"
        },
        {
          "name": "call",
          "type": "OpCode"
        },
        {
          "name": "amount",
          "type": "u128"
        },
        {
          "name": "time_lock",
          "type": "(u32, u32)"
        },
        {
          "name": "at",
          "type": "Hash",
          "isOptional": true
        }
      ],
      "type": "string"
    }
  }
}
```
