//THIS MODULE MANAGES THE BLOCKCHAIN AND VALIDATION LOGIC.
use crate::block::Block;    // This line imports Block from block module

pub struct Blockchain {     // Creates a public struct Blockchain
  pub chain: Vec<Block>,    // A vector of blocks making up the chain
}

impl Blockchain {
  // Initialize a new blockchain with the genesis block
  pub fn new() -> Self {
    let genesis_block = Block::new(0, "Genesis block".to_string(), "0".repeat(64));
    Blockchain {
      chain: vec![genesis_block],
    }
  }

  // Get the latest block in the chain
  pub fn latest_block(&self) -> &Block {
    self.chain.last().unwrap()
  }

  // Add a new block to the chain
  pub fn add_block(&mut self, data: String) {
    let last_block = self.latest_block();
    let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone());
    self.chain.push(new_block);
  }

  // Validate the integrity of the entire chain
  pub fn is_valid(&self) -> bool {
    for i in 1..self.chain.len() {
      let current = &self.chain[i];
      let previous = &self.chain[i - 1];

      // Check current block's hash
      let recalculated_hash =Block::calculate_hash(current.index, &current.timestamp, &current.data, &current.previous_hash);

      // If hashes don't match, the chain is invalid
      if current.hash != recalculated_hash {
        return false;
      }

      //Ensure current block links to the correct previous block
      if current.previous_hash != previous.hash {
        return false;
      }
    }
    true    // Return true if everything works fine.
  }

}