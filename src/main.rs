use accumulator::{utils::hash, Merkle, MerkleProof, NomadTree};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use ethers::abi::AbiEncode;
use ethers::types::*;
use rustc_hex::ToHex;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long)]
    /// The string to be added to the tree. Multiple can be supplied
    message: Vec<String>,
    #[clap(short, long)]
    /// The index of the leaf for which the proof is generated
    index: usize,
    #[clap(short, long)]
    /// Print Debug information
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // Create a Sparse Merkle Tree with depth 32
    // Insert the message to the tree
    let messages = args.message;
    let mut tree = NomadTree::default();
    if messages.is_empty() {
        return Err(anyhow!("You need to supply at least a message to be inserted in the tree\nType accumulator-cli -h for help"));
    }
    for message in &messages {
        tree.ingest(hash(message))
            .context(format!("Accumulator can't ingest {}", message))?;
    }
    // Calculate the Proof struct for the message at index 0
    let index = args.index;
    let proof = tree.prove(index).context(format!(
        "Accumulator can't generate proof for message at index {}",
        index
    ))?;
    let leaf = proof.leaf.to_fixed_bytes();
    let root = proof.root().to_fixed_bytes();
    let index: U256 = proof.index.into();
    // The path is an array of 32 hashes as FixedBytes (bytes32 in solidity).
    let path = proof.path.map(|hash| hash.to_fixed_bytes());
    // Abi-Encode the data
    // For quick reference, here is how Rust types are abi-encoded to Solidity types.
    // [u8; X] -> bytesX
    // Vec<u8> -> bytes (dynamic)
    // String  -> string (dynamic)
    // Bool    -> bool
    // u64     -> uint64 (etc. for other numbers)
    let encoded = (root, leaf, index, path).encode().to_hex::<String>();
    if args.debug {
        println!(
            r#"
//////////////////////////////////////////////////////////////
                           DEBUG INFO
//////////////////////////////////////////////////////////////
"#
        );
        println!(
            "🌴TREE\nMessages: {:?}\nRoot: {}\n\nPROOF\nRequested Leaf: {}\nRequested Index: {}\nPath: \n{}",
            messages,
            root.to_hex::<String>(),
            leaf.to_hex::<String>(),
            index,
            path.into_iter()
                .enumerate()
                .map(|(i, x)| format!("{:02}: {}\n", i, x.to_hex::<String>()))
                .collect::<String>()
        );
        println!(
            r#"
//////////////////////////////////////////////////////////////
                       ABI-ENCODED OUTPUT
//////////////////////////////////////////////////////////////
"#
        );
    }
    println!("{encoded}");
    Ok(())
}
