# MinixChain

## generate your keystore & chain spec.
```./target/release/chain-spec-builder generate --help```
Example:
```
    ./target/release/chain-spec-builder generate -a 2 -e 5GWw6ax784JGxQBcon9yzGXbvhZwxQhRiLvgxwRUST95W38z -k keystore -s 5GWw6ax784JGxQBcon9yzGXbvhZwxQhRiLvgxwRUST95W38z
```
Copy keystore to your chains db directory.
Set chain-id= chain_spec
Example:
```
./target/release/minix -d .1 --chain=./tmp/chain_spec.json --validator
```
