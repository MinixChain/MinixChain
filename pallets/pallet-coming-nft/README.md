# pallet-coming-nft

## Overview
pallet-coming-nft is a spin-off of the related NFT business from the original pallet-coming-id.
It is a collection of NFT operations based on Cid.

```rust
#[pallet::config]
pub trait Config: frame_system::Config + pallet_coming_id::Config {
    /// The implement of ComingNFT triat, eg. pallet-coming-id
    type ComingNFT: ComingNFT<Self::AccountId>;
    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}
```

Inherit `pallet_coming_id::Config` for `benchmarking`.

## Intro
- mint(cid, card):
**admin** permission, for the cid mint c-card.
  - If the cid is not assigned, an error is reported.
  - If cid has mint card, an error will be reported.
    
- transfer(cid, recipient):
**user** permission(owner), only 6-12 bit cid is allowed to transfer freely.
  - transfer to self = do nothing.
  - clear CidToApprove

- burn(cid):
**highAdmin** permission, only allow 1-5 cid to be destroyed.
  - If the cid is 6-12 bits, an error is reported
  - If the cid is invalid, an error is reported 
  - If the cid is not registered, an error will be reported

- approve(approved, cid):
**user** permission(owner), only allows 6-12 bit cid to freely approve.
  - After transfer or transfer_from, clear CidToApprove.

- set_approval_for_all(operator, flag):
**user** permission(owner),
  - Reference ERC721, exists independently of Cid
  - Delegate the ownership of all NFTs of the owner to the operator or cancel the operator's proxy authority
    
- transfer_from(from, to, cid):
**user** permission(operator)
  - The operator transfers the cid of from to to
  - clear CidToApprove
