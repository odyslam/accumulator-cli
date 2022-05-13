use std::str::FromStr;

use accumulator::{Tree, Proof, Merkle, MerkleProof};
use ethers::abi::token::Tokenizer;
use ethers::abi::{decode, encode, token, ParamType, param_type, Token, FixedBytes, Tokenize};
use ethers::types;
use primitive_types::H256;
use clap::Parser;
use std::str;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    message: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
    #[clap(parse(from_flag))]
    root: bool,
    #[clap(parse(from_flag))]
    proof: bool
}


fn main() {
    let args = Args::parse();
    let mut tree: Tree<32> = Default::default();
    tree.ingest(H256::from_str(&args.message).unwrap()).unwrap();
    let proof = tree.prove(0).unwrap();
    let leaf = proof.leaf.to_string();
    let index = proof.index.to_string();
    let path = proof.path.map(|hash| hash.to_string());
    let token3 = Token::FixedBytes(path.iter().map(|hash| hash.as_bytes()).collect<Vec<u8>>());
    let tokens = [Tokenizer::tokenize(&ParamType::String, &leaf).unwrap(), Tokenizer::tokenize(&ParamType::Uint(256), &index).unwrap(), token3];
    let encoded_output = str::from_utf8(&encode(&tokens)).unwrap();
    println!("{encoded_output}");
}
