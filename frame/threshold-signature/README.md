# pallet-threshold-signature

## Overview
`pallet-threshold-signature` is an m-of-n threshold signature module that combines Schnorr signature and MAST (Merkelized Abstract Syntax Tree) implementation. This module mainly implements threshold signature address generation and verification.

## Storage
- `AddrToScript`: is a Map that stores the mapping of threshold signature addresses to scripts.

## Call

- `generate_address(origin, scripts)`

  Generate threshold signature address according to the script provided by the user.

  `scripts`: The first parameter is inner pubkey. The remaining parameters are other scripts. For example, inner pubkey can be the aggregate public key of ABC, and other scripts can be the aggregate public key of AB, BC and AC.

- `verify_threshold_signature(origin, addr, signature, script, message, call)`

  Verify the threshold signature address and then call other transactions.

  `addr`: Represents a threshold signature address. For example, the aggregate public key of ABC
  `signature`: Usually represents the aggregate signature of m individuals. For example, the aggregate signature of AB
  `script`: Usually represents the aggregate public key of m individuals. For example, the aggregate public key of AB
  `message`: Message used in the signing process.
  `call`: The transaction that needs to be called after the threshold signature verification is passed.

## Tests

Refer to the [mock runtime](src/mock.rs) and [provided tests](src/tests.rs) to see the implementation in action.

## Example

To test the availability of the above two Calls, you need to use [musig](https://github.com/w3f/schnorrkel/blob/master/src/musig.rs#L780-L829) under the chain of [w3f/schnorrkel](https://github.com/w3f/schnorrkel) generates a set of [test data](https://github.com/chainx-org/threshold_signature/issues/1#issuecomment-909896156).

### Test Data

Aggregate public key of A, B, C

~~~
0x881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768
~~~

Aggregate public key of A, B

~~~
0x7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861
~~~

Aggregate public key of A, C

~~~
0xb69af178463918a181a8549d2cfbe77884852ace9d8b299bddf69bedc33f6356
~~~

Aggregate public key of B, C

~~~
0xa20c839d955cb10e58c6cbc75812684ad3a1a8f24a503e1c07f5e4944d974d3b
~~~

Message used to generate signature

~~~
0x576520617265206c6567696f6e21
~~~

A, B's aggregate signature of the above message

~~~
0x7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f
~~~

### Generate Address

![](https://cdn.jsdelivr.net/gh/AAweidai/PictureBed@master/taproot/1631104111907-1631104111872.png)

Use the buttons in the red box to fill in in order `Aggregate public key of A, B, C`, `Aggregate public key of A, B`, `Aggregate public key of A, C`, `Aggregate public key of B, C`.  You can generate a threshold signature address

![](https://cdn.jsdelivr.net/gh/AAweidai/PictureBed@master/taproot/1631104141760-1631104141749.png)

As shown in the figure above, it can be seen from the event that the address of the generated threshold signature is **`5Pe8v2KPm5dfdgRPDjAWdBSmWva7aeEH5nbZpYsHBX3mAVPK`**

### Verify Threshold Signature

While verifying the threshold signature, other transaction calls can be made. The other transactions tested here are transfers from the threshold signature address **`5Pe8v2KPm5dfdgRPDjAWdBSmWva7aeEH5nbZpYsHBX3mAVPK`** to other accounts. Therefore, it is necessary to transfer balance to the threshold signature address first, so that the address has a balance before the transfer can be performed.

![](https://cdn.jsdelivr.net/gh/AAweidai/PictureBed@master/taproot/1631104610241-1631104610236.png)

The picture above is the transfer to the threshold signature address **`5Pe8v2KPm5dfdgRPDjAWdBSmWva7aeEH5nbZpYsHBX3mAVPK`**

![](https://cdn.jsdelivr.net/gh/AAweidai/PictureBed@master/taproot/1631104780656-1631104780649.png)

The figure above is the operation interface for verifying the threshold signature. Fill in `addr` with **`5Pe8v2KPm5dfdgRPDjAWdBSmWva7aeEH5nbZpYsHBX3mAVPK`**. Fill in `signature` with `A, B's aggregate signature of the above message`. Fill in the `script` with `Aggregate public key of A, B`. Fill in `Message used to generate signature` in `message`. After the submission is successful, the operation of A and B co-signature transfer 100 from the threshold signature address to FERDIE is completed here.
