use std::fmt::{ self, Debug, Formatter };
use super::*;

// This is the block type.
// It is used to represent the block in the block chain.
// It contains the hash of the previous block, the hash of the block itself,
// the timestamp of the block, the nonce of the block, the difficulty of the block,
// and a list of transactions.
// The block is used to store the information of the block in the blockchain.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

// Debug trait for the block. It is used to print the block.
impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
               &self.index,
               &hex::encode(&self.hash),
               &self.timestamp,
               &self.transactions.len(),
               &self.nonce,
        )
    }
}

// Implement Block methods
impl Block {
    // Constructor for the block.
    // It takes the index, the timestamp, the previous block hash, the transactions and the difficulty as parameters.
    // It returns the block.
    // # Example
    // ```
    // use blockchainlib::*;
    // let block = Block::new(0, now(), vec![0; 32], vec![
    //         Transaction {
    //             inputs: vec![ ],
    //             outputs: vec![
    //                 transaction::Output {
    //                     to_addr: "Alice".to_owned(),
    //                     value: 50,
    //                 },
    //                 transaction::Output {
    //                     to_addr: "Bob".to_owned(),
    //                     value: 7,
    //                 },
    //             ],
    //         },
    //     ], difficulty);
    // ```
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    // mine method is the hash mining method.
    // It takes the block as parameter.
    // It returns nothing.
    pub fn mine (&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

// Hashable implementation for the Block.
impl Hashable for Block {
    // Convert the block to bytes.
    // It takes the block as parameter.
    // It returns the bytes.
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

// check_difficulty verify the difficulty found on mine() method.
// It takes the hash and the difficulty as parameters.
// It returns true if the hash is equal to the difficulty of the block.
pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}