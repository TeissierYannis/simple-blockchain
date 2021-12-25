use super::*;
use std::collections::HashSet;

#[derive(Debug)]
// Error type for blockchain
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

// Blockchain
// A blockchain is a list of blocks.
// The first block is the genesis block.
// The genesis block is the only block that has no previous block.
// The following blocks are chained together.
// The last block is the most recent block.
// The first block is the oldest block.
// The blockchain is immutable.
pub struct Blockchain {
    // The list of blocks
    pub blocks: Vec<Block>,
    // The list of transactions
    unspent_outputs: HashSet<Hash>,
}

// Blockchain implementation
impl Blockchain {
    // Blockchain constructor
    // Creates a new blockchain with empty blocks and no transactions.
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    // Verify a block
    // Verifies a block by checking the following:
    // - The index of the block is the same as the index of the previous block
    // - The hash of the block is the same as the hash of the previous block
    // - The timestamp of the block is greater than the timestamp of the previous block
    // - The previous hash of the block is the same as the hash of the previous block
    // - The coinbase transaction of the block is valid
    // - The transactions of the block are valid
    // - The sum of the input values of the transactions is equal to the sum of the output values of the transactions
    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        // Define blockchain length
        let i = self.blocks.len();

        // Check if the index of the block is the same as the index of the previous block
        if block.index != i as u32 {
            // Return error
            return Err(BlockValidationErr::MismatchedIndex);
        // Check if hash difficulty is valid
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            // Return error
            return Err(BlockValidationErr::InvalidHash);
        // Check if the index of the block is different than 0
        } else if i != 0 {
            // Not genesis block
            // Define previous block
            let prev_block = &self.blocks[i - 1];
            // Check if the timestamp of the block is greater than the timestamp of the previous block
            if block.timestamp <= prev_block.timestamp {
                // Return error
                return Err(BlockValidationErr::AchronologicalTimestamp);
            // Check if the previous hash of the block is the same as the hash of the previous block
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block (index = 0) if the hash of the genesis block is not empty
            if block.prev_block_hash != vec![0; 32] {
                // Return error
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        // Check if the coinbase transaction of the block is valid
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            // If the coinbase transaction is not valid
            if !coinbase.is_coinbase() {
                // Return error
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            // Hashes of input transactions
            let mut block_spent: HashSet<Hash> = HashSet::new();
            // Hashes of output transactions
            let mut block_created: HashSet<Hash> = HashSet::new();
            // rest of the transactions
            let mut total_fee = 0;

            // For each transactions
            for transaction in transactions {
                // get inputs
                let input_hashes = transaction.input_hashes();

                // Check if the transaction is valid
                if
                !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    // Return error
                    return Err(BlockValidationErr::InvalidInput);
                }

                // add inputs to spent
                let input_value = transaction.input_value();
                // add outputs to created
                let output_value = transaction.output_value();

                // if the input value is less than the output value
                if output_value > input_value {
                    // Return error
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                // rest of the input value is the fee
                let fee = input_value - output_value;

                // add fee to total fee
                total_fee += fee;

                // add inputs to spent
                block_spent.extend(input_hashes);
                // add outputs to created
                block_created.extend(transaction.output_hashes());
            }

            // output value of coinbase transaction is inferior to the total fee that means the coinbase transaction is not valid
            if coinbase.output_value() < total_fee {
                // Return error
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                // Add the coinbase transaction to the created block
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        // Add the block to the chain
        self.blocks.push(block);

        // Return success
        Ok(())
    }
}