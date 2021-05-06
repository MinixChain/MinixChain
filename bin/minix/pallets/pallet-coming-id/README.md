# pallet-coming-id

## Overview
`pallet-coming-id` 是 `MinixChain` 上用来标识`Coming App`的用户身份的链上代理系统. 

每个`coming-id`可绑定指定类型的数据(如btc地址,eth地址,合约地址,传统互联网帐号等).

每个`coming-id`同一时间有且只有一个属主(`Substrate公私钥体系`).

## Intro
- `pallet-coming-id`(简称`cid`)由1-12位数字组成

   [1,100000)为ChainX内部预留, 
   
   [100000,1000000) 为ChainX 社区预留, 
   
   [1000000,100000000000)所有用户均可申领.

- `cid`的分配权和转移权:  
  - 分配权: 
  
    ChainX拥有所有cid的分配权;
  
    对于[1000000,100000000000)可由所有用户申领,ChainX批准后拥有.
  - 转移权: 
  
    ChainX只拥有[1,100000)的转移权;
    
    其余cid的转移权归其属主拥有.
    
- 关键函数

  - register(cid, recipient): 
  
    admin权限, 分配 1-12位 cid.
  
  - approve(cid_start, cid_end): 
    
    admin权限, 批量批准cid申领请求.
  
  - disapprove(cid_start, cid_end): 
    
    admin权限, 批量拒绝cid申领请求.

  - claim(recipient):
     
      admin权限, 系统按递增顺序分配7-12位cid, ForceClaim, 不需要admin approve.
      
      user权限, 系统按递增顺序分配7-12位cid, Claiming, 需要admin approve.
     
      当claiming请求在一定时限`ClaimValidatePeriod`内, admin没有approve/disapprove,则cid被系统回收
  
    - transfer(cid, recipient)
     
      user权限(owner), 只允许6-12位cid自由transfer.
  
      transfer to self = unbond all
  
    - bond(cid, bond_data)
  
      user权限(owner),  对指定cid, bond数据(类型字段和数据字段):
  
      ```rust
      pub struct BondData {
         pub bond_type: BondType,
         pub data: Vec<u8>
      }
      ```
  
    - unbond(cid, bond_type)
   
      user权限(owner), unbond 指定cid, bond类型字段
