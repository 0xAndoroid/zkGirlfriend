//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use fibonacci_lib::verify_messages;

pub fn main() {
    let messages = sp1_zkvm::io::read::<Vec<u8>>();

    let score = verify_messages(messages);

    sp1_zkvm::io::commit_slice(&U256::from(score).abi_encode());
}
