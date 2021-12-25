use blockchainlib::*;

// Rust implementation of the blockchain library.
fn main () {
    // Set base difficulty.
    let difficulty = 0x00000fffffffffffffffffffffffffff;

    // Create a new genesis block.
    // The genesis block is automatically created.
    // The difficulty is set to the base difficulty.
    // the initial index is set to 0.
    // The initial timestamp is set to the current time.
    // The initial hash is set to the genesis block hash (0).
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        },
    ], difficulty);

    // Mine the block to confirm it.
    genesis_block.mine();

    // Debug block
    println!("Mined genesis block {:?}", &genesis_block);

    // Get the hash of the block
    let mut last_hash = genesis_block.hash.clone();

    // Create a new blockchain.
    let mut blockchain = Blockchain::new();

    // Verify the genesis block. and add it to the blockchain if it is valid.
    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    // Create a new block.
    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![],
        },
    ], difficulty);

    // Mine the block to confirm it.
    block.mine();

    // Debug block
    println!("Mined block {:?}", &block);

    // Get the last hash of the last block.
    last_hash = block.hash.clone();

    // Verify the block. and add it to the blockchain if it is valid.
    blockchain.update_with_block(block).expect("Failed to add block");
}