# pallet-coming-reputation

## Overview
pallet-coming-reputation是Coming 声誉系统业务的实现

## Intro
- set_admin(new_admin): sudo权限, 设置新的管理员.
- up_grade(cid,grade): 管理员权限, 信誉升级.

## rpc 
- get_reputation_grade: 获取指定cid的信誉等级

输入：
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"get_reputation_grade",
  "params": [1000000]
}
```
输出：
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