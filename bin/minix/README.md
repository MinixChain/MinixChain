# MiniX

## Intro
`pallet-coming-id`, `pallet-coming-nft`, `pallet-coming-auction`

## types
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
  }
}
```

## rpc types
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
    }
}
```
