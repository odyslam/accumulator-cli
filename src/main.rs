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
    let mut tree: Tree<32> = Default::default();
    tree.ingest(hash(args.message)).unwrap();
    let proof = tree.prove(0).unwrap();
    let leaf: String = proof.leaf.to_string();
    let index: U256 = proof.index.into();
    let encoded = (leaf, index).encode().to_hex::<String>();
    println!("{}", encoded);
}

