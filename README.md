# Wrapped Appchain NFT

This is an implementation of wrapped appchain NFT contract of Octopus Network.

This contract is for supporting cross-chain NFT transfer from Octopus Appchain to NEAR protocol. It is used in [Octopus Appchain Anchor](https://github.com/octopus-network/octopus-appchain-anchor) since `v2.1.0`.

For those Octopus Appchains which supports NFT assets, each of the `class id` in appchain will has one instance of this contract in NEAR protocol. All the instance of this contract for a specific appchain will be only controlled by the corresponding `Octopus Appchain Anchor`.

The implementation of this contract references the [NFT sample of near-sdk-rs v4.0.0](https://github.com/near/near-sdk-rs/tree/master/examples/non-fungible-token).

## Building

To build run:

```bash
./build.sh
```

## Testing

To test run:

```bash
cargo test --workspace --package wrapped-appchain-nft -- --nocapture
```
