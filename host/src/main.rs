pub mod input;

use anyhow::Result;
use input::read_input;
use methods::{RISC0_GUEST_ELF, RISC0_GUEST_ID};
use risc0_core::{Inputs, Page};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

/// Given a merkle tree root hash and memory pages, computes the merkle tree
/// inside the zkVM and returns a receipt, including a journal and seal
/// attesting to the fact that the commited root hash over the committed pages
/// is correct.
fn prove_merkle_root(inputs: &Inputs) -> Receipt {
    let env = ExecutorEnv::builder()
        .write(&inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove(env, RISC0_GUEST_ELF).unwrap().receipt
}

fn main() -> Result<()> {
    // read input data
    let inputs = read_input()?;

    // run merkle root verification inside the zkVM guest and get the resulting receipt
    let receipt = prove_merkle_root(&inputs);

    // verify the receipt and access the journal
    receipt.verify(RISC0_GUEST_ID).unwrap();
    let (merkle_root, _pages): (Vec<u8>, Vec<Page>) =
        receipt.journal.decode::<(Vec<u8>, _)>().unwrap();

    println!("Merkle root verified: {:?}", merkle_root);
    Ok(())
}
