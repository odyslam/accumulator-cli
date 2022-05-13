# Sparse Merkle Tree Accumulator

It's a cli version of the [Nomad Accumulator](https://github.com/nomad-xyz/rust/tree/main/accumulator).

It's very limited, as it's primarily used in our Forge testing suite via the [-ffi](https://book.getfoundry.sh/forge/differential-ffi-testing.html) cheatcode.

In essence, we generate proofs via the rust binary for arbitrary inputs and then feed these proofs in the solidity code to be verified. This  simulates the way our off-chain agents generate proofs and then submit them to the smart contracts.


## License

MIT
