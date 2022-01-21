# pallet-coming-auction

## Overview
pallet-coming-auction是Coming NFT拍卖业务的实现,主要采用
荷兰式拍卖(start_price > end_price). 
如果设置了admin和fee rate([0, 2.55%], 最小可调fee point是万分之一),
将对拍卖的买方收取相应的手续费.

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
继承`pallet_coming_id::Config`是为了`benchmarking`.

## Intro
- create(cid, start_price, end_price, duration):
    普通权限, 用户创建一个auction,
    cid: 要拍卖的cid
    start_price: 至少大于最小balance
    end_price: 至少大于最小balance
    duration: 从start_price渐变到end_price的持续时间(以block数计算),最小值是100(10分钟)
    
- bid(cid, value):
    普通权限, 用户拍卖一个auction,
    出价大于等于系统的报价即拍卖成功,该拍卖结束
    cid: 要拍卖的cid
    value: 此次拍卖出价

- cancel(cid):
    普通权限, 用户取消正在进行的auction(即使处于紧急状态也可以取消)
    cid: 正在进行拍卖的cid
    
- pause():
    管理员权限, 紧急按键, 暂停`auction`, `bid`, `set_fee_point`
    
- unpause():
    管理员权限, 撤销紧急按键. 恢复`auction`, `bid`, `set_fee_point`
    
- cancel_when_pause(cid):
    管理员权限, 紧急状态下让管理员取消指定cid的拍卖.
    
- set_fee_point(new_point):
    管理员权限, 设置新的服务费率, [0, 255]分别对应0到万分之二百五十五.
    
- set_admin(new_admin):
    sudo权限, 设置新的管理员.
    
- remint:
    普通权限, NFT二次创作. `remint fee`随remint次数2倍增加.
- set_remint_point:
    管理员权限, 设置`remint`费用调节因子, 调节范围[0%, 255%].

## fee

- `transaction_payment`: 交易手续费, 转给系统销毁
- `service_fee`: 拍卖服务费, 转给NFT拍卖平台admin
- `tax_fee`: NFT拍卖版税, 转给NFT创作者issuer
- `remint_fee`: NFT二次创作费, 转给NFT拍卖平台admin

## rpc 
- get_price: 获取指定cid的当前拍卖价格
```rust
#[rpc(name = "get_price")]
    fn get_price(
        &self,
        cid: Cid,
        at: Option<BlockHash>
    ) -> Result<NumberOrHex>;
```
输入：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_price",
  "params": [1000000]
}
```
输出：
```json
{
  "jsonrpc": "2.0",
  "result": "0x594ff81df000",
  "id": 1
}
```

- get_remint_fee: 获取指定cid的当前`remint fee`
```rust
#[rpc(name = "get_remint_fee")]
    fn get_remint_fee(
        &self,
        cid: Cid,
        at: Option<BlockHash>
    ) -> Result<NumberOrHex>;
```
输入：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_remint_fee",
  "params": [1000000]
}
```
输出：
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
