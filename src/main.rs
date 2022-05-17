use std::{str::FromStr, default};

use accumulator::{Tree, Proof, Merkle, MerkleProof, utils::hash};
use ethers::abi::{AbiParser, encode, Token, AbiEncode, FixedBytes};
use ethers::types::*;
use clap::Parser;
use std::str;
use rustc_hex::ToHex;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    message: String,
    #[clap(parse(from_flag))]
    root: bool,
    #[clap(parse(from_flag))]
    proof: bool
}


fn main() {
    let args = Args::parse();
    // Create a Sparse Merkle Tree with depth 32
    let mut tree: Tree<32> = Default::default();
    // Insert the message to the tree
    tree.ingest(hash(args.message)).unwrap();
    // Calculate the Proof struct for the message at index 0
    let proof = tree.prove(0).unwrap();
    let leaf = proof.leaf.to_fixed_bytes();
    let index: U256 = proof.index.into();
    // The path is an array of 32 hashes as FixedBytes (bytes32 in solidity).
    let path = proof.path.map(|hash| hash.to_fixed_bytes());
    // Abi-Encode the data.
    // For quick reference, here is how Rust types are abi-encoded to Solidity types.
    // [u8; X] -> bytesX
    // Vec<u8> -> bytes (dynamic)
    // String -> string (dynamic)
    // bool -> bool
    // u64 -> uint64 (etc. for other numbers)
    let encoded = (leaf, index, path).encode().to_hex::<String>();
    println!("{encoded}");
}

