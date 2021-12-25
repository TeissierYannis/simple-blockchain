use super::*;
use std::collections::HashSet;

// In a blockchain, a transaction is a set of instructions that can be applied to a ledger.
// Transactions are signed by a sender, and can be verified by the sender's public key.
// Transactions are immutable, and can be applied to a ledger only once.
// Transactions are applied to a ledger in the order they are received.
//
// Outputs are what we send to other people, and inputs are what we spent.

#[derive(Clone)]
// Output transaction is a value that is sent to the recipient.
// It is a part of the transaction.
// to_address is the address of the recipient.
// value is the value of the output transaction.
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

// Hashable for Output
impl Hashable for Output {
    // Convert output to bytes
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

// Transaction is a collection of inputs and outputs.
// Inputs are the transactions that are used to pay for the outputs.
// Outputs are the transactions that are sent to the recipients.
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

// Implementation of transaction
impl Transaction {
    // sum of inputs
    pub fn input_value (&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    // sum of outputs
    pub fn output_value (&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    // Hashset of inputs
    pub fn input_hashes (&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    // hashes of outputs
    pub fn output_hashes (&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    // verify that the transaction is a coinbase transaction (no inputs)
    pub fn is_coinbase (&self) -> bool {
        self.inputs.len() == 0
    }
}

// Implementation of transaction
impl Hashable for Transaction {
    // Convert transaction to bytes
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}