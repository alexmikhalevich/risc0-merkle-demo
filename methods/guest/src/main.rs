use risc0_zkvm::guest::env;
use risc0_core::{Inputs, Page};
use tiny_keccak::{Hasher, Keccak};


fn compute_merkle_root(pages: &Vec<Page>) -> Vec<u8> {
    let mut leaves: Vec<Vec<u8>> = pages
        .iter()
        .map(|page| {
            let mut hasher = Keccak::v256();
            let mut output = [0u8; 32];
            hasher.update(&page.data);
            hasher.finalize(&mut output);
            output.to_vec()
        })
    .collect();

    while leaves.len() > 1 {
        let mut new_leaves = Vec::new();
        for i in (0..leaves.len()).step_by(2) {
            let mut hasher = Keccak::v256();
            let mut output = [0u8; 32];
            hasher.update(&leaves[i]);
            if i + 1 < leaves.len() {
                hasher.update(&leaves[i + 1]);
            }
            hasher.finalize(&mut output);
            new_leaves.push(output.to_vec());
        }
        leaves = new_leaves;
    }
    leaves[0].clone()
}

fn main() {
    // read the input
    let inputs: Inputs = env::read();
    let origin_merkle_root = inputs.merkle_root;

    // compute the merkle root
    let computed_merkle_root = compute_merkle_root(&inputs.pages);

    // verify the merkle root, panic if it is invalid
    assert_eq!(origin_merkle_root, computed_merkle_root, "Merkle root is invalid");

    // commit to the journal the correct merkle root and the pages
    env::commit(&(computed_merkle_root, inputs.pages));
}
