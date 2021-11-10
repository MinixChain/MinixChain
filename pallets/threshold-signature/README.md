# pallet-threshold-signature

## Overview
`pallet-threshold-signature` is an m-of-n threshold signature module that combines Schnorr signature and MAST (Merkelized Abstract Syntax Tree) implementation. This module mainly supports verifying signatures, authorizing scripts and executing scripts.

## Storage
- `ScriptHashToAddr`: is a Map that stores the mapping of script hash to threshold signature address.

## Types

~~~json
{
 "OpCode": {
    "_enum": [
      "Transfer"
    ]
  }
}
~~~

# Rpc

- `ts_computeScriptHash`

  ~~~rust
  fn compute_script_hash(   
    &self,
    account: AccountId32,
    call: OpCode,
    amount: u128,
    time_lock: (u32, u32),
    at: Option<BlockHash>,
  ) -> Result<String>;
  ~~~

## Call

- `pass_script(origin, addr, signature, pubkey, control_block, message, script_hash)`

  Verify the multi-signature address and authorize to the script represented by the script hash.

  `addr`: Represents a threshold signature address. Calculated by merkle root and inner pubkey.   
  
  `signature`: Usually represents the aggregate signature of m individuals. For example, the aggregate signature of AB.    
  
   `pubkey`: Usually represents the aggregate public key of m individuals. For example, the aggregate public key of AB
  
  `control_block`: The first element is inner pubkey, and the remaining elements are merkle proof. For example, merkle proof may be `[tag_hash(pubkey_BC), tag_hash(pubkey_AC)]`.         
  
  `message`: Message used in the signing process, it is also the block height where the signature exists.       
  
  `call`: The transaction that needs to be called after the threshold signature verification is passed.   

- `exec_script(origin, call, amount, time_lock)`

  The user takes the initiative to execute the truly authorized script.      
  `origin`: Signed executor of the script. It must be `pass_script` to complete the script authorized to the user before the user can execute successfully    

  `target`: Receiver address.

  `call`: Action represented by the script.    

  `amount`: The number represented by the script.    

  `time_lock`: Time lock required for script execution. The script must meet the time lock limit before it can be executed successfully      

## Tests

Refer to the [mock runtime](src/mock.rs) and [provided tests](src/tests.rs) to see the implementation in action.

## Example

To test the availability of the above two Calls, you need to use [musig](https://github.com/w3f/schnorrkel/blob/master/src/musig.rs#L780-L829) under the chain of [w3f/schnorrkel](https://github.com/w3f/schnorrkel) generates a set of [test data](https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156).

### Test Data

#### Basic data

1. Aggregate public key of A, B

~~~
0x744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b
~~~

2. Threshold address

~~~
0x3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719
~~~

3. Message used to generate signature

~~~
666666
~~~

4. A, B's aggregate signature of the above message

~~~
0x98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88
~~~

5. Control block

~~~
0xfa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634
~~~

6. Alice's pubkey

~~~
0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
~~~

#### Generate data by basic data

1. Generate threshold signature address off-chain

~~~
pubkey_abc    +     root
                   /    \
               branch   AC
               /    \
              AB    BC
~~~

​	As shown in the figure above, the address is generated according to the formula `P + hash_tweak(P||root)G`. The generated threshold signature address is:

~~~
5Pe8v2KPm5dfdgRPDjAWdBSmWva7aeEH5nbZpYsHBX3mAVPK
~~~

2. tag_hash(BC)

~~~
0xe17a23050f6f6db2f4218ce9f7c14edd21c5f24818157103c5a8524d7014c0dd
~~~

3. tag_hash(AC)

~~~
0x0bac21362eecf9223bc477d6dfbbe02066a911eba752faedb26d881c466ea80f
~~~

4. Script is consist of [Account, Call, Amount, TimeLock]. The script currently used is: `[Alice's pubkey, Transfer, 10, (0, 1000)]`. The meaning of this simple script is to transfer 10 from the threshold signature address to Alice under the restriction of TimeLock. For more detailed details, please refer to [Substrate-taproot](https://github.com/chainx-org/Substrate-taproot/blob/main/README.md).  **Script hash** :

~~~
0x2ad121d05a26705dfe7e8005d2a87f9c035c4f439d18f6b2a4fbae6cc6012734
~~~

### Pass Script

![](https://cdn.jsdelivr.net/gh/hacpy/PictureBed@master/Document/16365112850891636511285080.png)

Pass Script is the transaction call is to verify the threshold signature address and authorize the script. As shown in the figure above:

- Fill in `addr` with **`0x3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719`**. 
- Fill in `signature` with `A, B's aggregate signature of the above message`. 
- Fill in the `pubkey` with `Aggregate public key of A, B`. 
- Fill in `control_block` in order: [`Aggregate public key of A, B, C`,   `tag_hash(BC)`, `tag_hash(AC)`]
- Fill in `message` with `Message used to generate signature`
- Fill in `script_hash` with `Script hash`

After the submission is successful, the five parameters of `addr`, `signature`, `pubkey`, `control_block`, and `message` are used for threshold signature verification. After the verification is passed, the script hash will be written into the storage, that is, the corresponding script is authorized. 

### Exec Script

The above-mentioned authorized script is the operation of transferring money from a  threshold signature address `0x3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719` to Alice. Therefore, the balance needs to be transferred to the threshold signature address first, and then the transfer can be performed after the address has a balance.

![](https://cdn.jsdelivr.net/gh/AAweidai/PictureBed@master/taproot/1631104610241-1631104610236.png)

The picture above is the transfer to the threshold signature address **`0x3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719`**

![](https://cdn.jsdelivr.net/gh/hacpy/PictureBed@master/Document/1635133723155-1635133723152.png)

As shown in the figure above, when the block height is between 0 and 1000, Alice can actively execute the script. After the execution of the script is completed, 10 unit will be transferred to Alice from the threshold signature address.
