# pallet-coming-reputation

## Overview
pallet-coming-reputation is the implementation of Coming's reputation system business.

## Intro
- set_admin(new_admin): sudo permission,set up a new administrator.
- upgrade(cid,grade): admin permission, reputation upgrade.

## rpc 
- get_reputation_grade: Get the credit rating of the specified cid

input：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_reputation_grade",
  "params": [1000000]
}
```
output：
```json
{
  "jsonrpc": "2.0",
  "result": {
    "key1": 9,
    "key2": 0,
    "key3": 0
  },
  "id": 1
}
```