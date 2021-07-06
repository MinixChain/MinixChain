# pallet-coming-nft

## Overview
pallet-coming-nft 是从原pallet-coming-id中将相关的NFT业务剥离出来的,
是以Cid为基础的NFT操作集合.

## Intro
- mint(cid, card): admin权限
    
    为该cid mint c-card.
    
    如果cid未分配,则报错.
    
    如果cid已mint card,则报错.
    

- transfer(cid, recipient): 
    
    user权限(owner), 只允许6-12位cid自由transfer.
    
    transfer to self = unbond all.
