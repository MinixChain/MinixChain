# pallet-coming-auction

## Overview
pallet-coming-auction is the realization of Coming NFT auction business, 
mainly using Dutch auction(start_price > end_price).
If admin and fee rate([0, 2.55%], The minimum adjustable fee point is one ten thousandth) are set,
a corresponding handling fee will be charged to the buyer of the auction.

![image](https://user-images.githubusercontent.com/8869892/132611008-4b39b11c-51f7-4d21-9707-4b59ceb1a59a.png)


![image](https://user-images.githubusercontent.com/8869892/132611596-f7704a24-97dc-4b94-94ef-d869ef7a49dd.png)



```rust
#[pallet::config]
    pub trait Config: frame_system::Config + pallet_coming_id::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The implement of ComingNFT triat, eg. pallet-coming-id
        type ComingNFT: ComingNFT<Self::AccountId>;
        /// The native balance
        type Currency: Currency<Self::AccountId>;
        /// This pallet id.
        #[pallet::constant]
        type PalletId: Get<PalletAuctionId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }
```
Inherit `pallet_coming_id::Config` for `benchmarking`.

## Intro
- create(cid, start_price, end_price, duration):
  Ordinary permissions, the user creates an auction,
    cid: cid to auction
    start_price: At least greater than the minimum balance
    end_price: At least greater than the minimum balance
    duration: The duration of the gradient from start_price to end_price (calculated in blocks), the minimum value is 100 (10 minutes)
    
- bid(cid, value):
  Ordinary permissions, the user creates an auction,
  If the bid is greater than or equal to the system's bid, the auction is successful and the auction ends
    cid: cid to auction
    value: Bids in this auction

- cancel(cid):
  Ordinary permissions, the user can cancel the ongoing auction (even in an emergency state)
    cid: cid to auction
    
- pause():
  admin rights, panic button, pause `auction`, `bid`, `set_fee_point`
    
- unpause():
  Admin privileges, revoke panic button. Restore `auction`, `bid`, `set_fee_point`
    
- cancel_when_pause(cid):
  Administrator privilege, in emergency state, let the administrator cancel the auction of the specified cid.
    
- set_fee_point(new_point):
  Administrator rights, set a new service rate, [0, 255] corresponds to 0 to 255/10,000 respectively.
    
- set_admin(new_admin):
  sudo privileges, set up a new administrator.
    
- remint:
  Ordinary permissions, secondary creation of NFT. `remint fee` increases by 2 times with the number of remints.
  The weight of each byte is `W0`,
    When `card_size` <= 1024 bytes, `card_weights` = `W0`;
    When `card_size` > 1024 bytes, `card_weights` = `card_size * W0`; 
    card_size max `1024 * 1024` bytes.
- set_remint_point:
    Administrator rights, set `remint` cost adjustment factor, adjustment range [0%, 255%].

## fee

- `transaction_payment`: Transaction fee, transferred to the system for destruction
- `service_fee`: Auction service fee, transfer to NFT auction platform admin
- `tax_fee`: NFT auction royalties, transferred to NFT creator issuer
- `remint_fee`: The NFT secondary creation fee will be transferred to the NFT auction platform admin

## rpc 
- get_price: Get the current auction price of the specified cid
```rust
#[rpc(name = "get_price")]
    fn get_price(
        &self,
        cid: Cid,
        at: Option<BlockHash>
    ) -> Result<NumberOrHex>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_price",
  "params": [1000000]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": "0x594ff81df000",
  "id": 1
}
```

- get_remint_fee: Get the current `remint fee` for the specified cid
```rust
#[rpc(name = "get_remint_fee")]
    fn get_remint_fee(
        &self,
        cid: Cid,
        at: Option<BlockHash>
    ) -> Result<NumberOrHex>;
```
enter：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_remint_fee",
  "params": [1000000]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": "0x2faf080",
  "id": 1
}
```

## custom types

```json
{
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

## rpc custom

```json

{
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
    },
    "getRemintFee": {
      "description": "comingAuction getRemintFee",
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
