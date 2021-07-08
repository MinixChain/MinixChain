# pallet-coming-nft

## Overview
pallet-coming-nft 是从原pallet-coming-id中将相关的NFT业务剥离出来的,
是以Cid为基础的NFT操作集合.

```rust
#[pallet::config]
pub trait Config: frame_system::Config + pallet_coming_id::Config {
    /// The implement of ComingNFT triat, eg. pallet-coming-id
    type ComingNFT: ComingNFT<Self::AccountId>;
    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}
```

继承`pallet_coming_id::Config`是为了`benchmarking`.

## Intro
- mint(cid, card): 

    admin权限, 为该cid mint c-card.
    
    如果cid未分配,则报错.
    
    如果cid已mint card,则报错.
    

- transfer(cid, recipient): 
    
    user权限(owner), 只允许6-12位cid自由transfer.
    
    transfer to self = unbond all.

- burn(cid):
    high admin权限, 只允许销毁1-5位cid.
    
    如果cid是6-12位,则报错
    
    如果cid无效,则报错
    
    如果cid未register,则报错
