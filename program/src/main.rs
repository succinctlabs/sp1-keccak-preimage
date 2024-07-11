//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha3::{Digest, Keccak256};

pub fn main() {
    let preimage = sp1_zkvm::io::read_vec();
    let offset_u32 = sp1_zkvm::io::read::<u32>();
    let offset: usize = offset_u32.try_into().unwrap();

    let mut hash = [0u8; 32];
    let mut keccak256 = Keccak256::new();
    keccak256.update(&preimage);
    hash.copy_from_slice(&keccak256.finalize());

    let mut chunk = Vec::new();
    if offset < preimage.len() {
        let chunk_end = std::cmp::min(offset.saturating_add(32), preimage.len());
        chunk = preimage[offset..chunk_end].to_vec();
    }

    sp1_zkvm::io::commit(&hash);
    sp1_zkvm::io::commit(&offset_u32);
    sp1_zkvm::io::commit(&chunk);
}
