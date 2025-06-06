//THIS IS THE ENTRY POINT OF THE BLOCKCHAIN PROGRAM, IT ALSO BUILDS AND RUNS THE BLOCKCHAIN PROGRAM.

mod block;        // The mod keyword calls and includes the block module
mod blockchain;   // Also includes the blockchain module


use blockchain::Blockchain;   // A path to import the structure in the blockchain module
use serde_json;               // For pretty-printing in JSON format

fn main() {                                             // Start of main function where our code starts to run
    let mut chain = Blockchain::new();      //This line creates a mutable blockchain with genesis block

    // Add some blocks to the chain
    chain.add_block("First block after genesis".to_string());
    chain.add_block("Second block with some data".to_string());
    chain.add_block("Third block with some data".to_string());
    chain.add_block("Last block with some data".to_string());

    println!("Is the blockchain valid? {}\n", chain.is_valid());       // Checks if blocks appended to the chain is valid

    //Uses a for loop to print the blockchain as JSON format
    for block in &chain.chain {
        println!("{}", serde_json::to_string_pretty(block).unwrap());
    }
}
