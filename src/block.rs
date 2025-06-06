//THIS MODULE DEFINES THE BLOCK STRUCTURE AND IT'S METHODS.

use chrono::Utc;                    // A path to Utc in chrono crate specified in our dependencies, to get the current timestamp of our blockchain program
use sha2::{Digest, Sha256};         // Also imports two items from the sha2 module which contains the hashing implementations
use serde::{Serialize, Deserialize};  // This allows blocks to be printed as JSON format

#[derive(Serialize, Deserialize, Debug, Clone)]       //Defining some attributes that is needed for foramtting and printing the Block structure
pub struct Block {         // Creates a public Block structure
  pub index: u64,          // Block number in the chain
  pub timestamp: String,   // Time the block was created
  pub data: String,        // Data stored in the block
  pub previous_hash: String, // Hash of the previous block
  pub hash: String,          // This block's hash
}

impl Block {    // Implementation of the Block structure

  //Create a new block
  pub fn new(index: u64, data: String, previous_hash: String) -> Self {                //The `new` method accepts some parameters and returns a Self which can also be called Block
    let timestamp = Utc::now().to_string();                                    //This line creates a variable to hold the timestamp and also converts it to String.
    let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash); //create a variable to hold the result of the block hash
    Block {
      index,
      timestamp,
      data,
      previous_hash,
      hash,
    }
  }

  // Compute SHA-256 hash for a block
  pub fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {  
    let mut hasher = Sha256::new();       // Creates an new variable `hasher` to hold the hasher values
    hasher.update(index.to_string());    // This line updates and pushes the required values to the hasher variable.
    hasher.update(timestamp);            // The `.update()` method is a method defined in the Digest trait from the sha2 crate.
    hasher.update(data);                 // It is used to feed data incrementally into a hashing algorithm.
    hasher.update(previous_hash);
    let result = hasher.finalize();      // Lastly the `.finalize()` method returns the final hash as a byte array. The variable `result` holds the final value.
    format!("{:x}", result)             // The format! macro prints and returns the value/values as a string. The line prints in a lowercase Hexadecimal format.

  }
} //End of implementation block