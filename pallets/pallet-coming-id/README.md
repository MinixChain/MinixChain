# pallet-coming-id

## Overview
`pallet-coming-id` is an on-chain proxy system on `MinixChain` used to identify the user identity of `Coming App`. 

Each `coming-id` can be bound to a specified type of data (such as btc address, eth address, contract address, traditional Internet account, etc.).

Each `coming-id` has one and only one owner at the same time (`Substrate public-private key system`).

## Intro
- `pallet-coming-id` (short for `cid`) consists of 1-12 digits

  [1,100000) is reserved internally for `Coming`,

  [100000,1000000) reserved for the `Coming` community,

  [1000000,100000000000) All users can apply.

- Distribution rights and transfer rights of `cid`:  
  - distribution rights: 

    `Coming` has the right to assign all cids;
    
  - transfer right: 

    `Coming` only has the transfer right of [1,100000);

    The transfer right of the remaining cid belongs to its owner.
    
- key function

  - register(cid, recipient): 

    highAdmin permissions, assign 1-12 digits of cid.

    mediumAdmin3 permissions, assign a 6-digit cid.
    mediumAdmin2 permissions, assign a 7-digit cid.
    mediumAdmin permissions, assign 8-digit cid.

    lowAdmin permissions, assign 9-12 digits of cid.
  
  - bond(cid, bond_data)

    User permission (owner), for the specified cid, bond data (type field and data field):
  
      ```rust
      pub struct BondData {
         pub bond_type: BondType,
         pub data: Vec<u8>
      }
      ```
  
  - unbond(cid, bond_type)

    user permission (owner), unbond specifies cid, bond type fields

## rpc
- get_account_id:
  Get the account id of the specified cid

```
#[rpc(name = "get_account_id")]
fn get_account_id(
   &self,
   cid: Cid,
   at: Option<BlockHash>
) -> Result<Option<AccountId>>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_account_id",
  "params": [1000000]
}
```
output1：
```json
{
  "jsonrpc": "2.0",
  "result": null,
  "id": 1
}
```
output2：
```json
{
  "jsonrpc": "2.0",
  "result": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
  "id": 1
}
```
- get_cids:
  Get the cids of the specified account id
```
#[rpc(name = "get_cids")]
fn get_cids(
   &self,
   account: AccountId,
   at: Option<BlockHash>
) -> Result<Vec<Cid>>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_cids",
  "params": ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": [
    99,
    999
  ],
  "id": 1
}
```
- get_bond_data:
  Get the bond data of the specified cid

```
#[rpc(name = "get_bond_data")]
fn get_bond_data(
    &self,
    cid: Cid,
    at: Option<BlockHash>
) -> Result<Option<CidDetails<AccountId>>>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_bond_data",
  "params": [99]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": {
    "bonds": [
      {
        "bondType": 1,
        "data": "0x7b226e616d65223a227465737432227d"
      }
    ],
    "card": "0x7b226e616d65223a202274657374227d",
    "owner": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
  },
  "id": 1
}
```

- get_card:
  Get the c-card of the specified cid

```
#[rpc(name = "get_card")]
fn get_card(
    &self,
    cid: Cid,
    at: Option<BlockHash>
) -> Result<Option<CidDetails<AccountId>>>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_card",
  "params": [99]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": "0x7b226e616d65223a202274657374227d",
  "id": 1
}
```

- get_card_meta:
  Get the metadata of the c-card of the specified cid: remint times, issuer, tax rate.
  tax rate range[0, 2.55%]

```
#[rpc(name = "get_card_meta")]
fn get_card_meta(
    &self,
    cid: Cid,
    at: Option<BlockHash>
) -> Result<Option<CardMeta<AccountId>>>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_card_meta",
  "params": [1000000]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": {
    "issuer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "remint": 1,
    "taxPoint": 50
  },
  "id": 1
}
```

## custom types
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
  "CardMeta": {
    "remint": "u8",
    "issuer": "AccountId",
    "tax_point": "u8"
  },
  "CidDetails": {
    "owner": "AccountId",
    "bonds": "Vec<BondData>",
    "card":  "Bytes",
    "card_meta": "Option<CardMeta>"
  },
  "AdminType": {
    "_enum": [
      "High",
      "Medium",
      "Medium2",
      "Medium3",
      "Low"
    ] 
  }    
}
```
## rpc custom 
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
    },
    "getCardMeta": {
      "description": "comingId getCardMeta",
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
      "type": "Option<CardMeta>"
    }
  }
}
```
