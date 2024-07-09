//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use tiny_keccak::{Hasher, Keccak};

pub fn main() {
    let preimage = sp1_zkvm::io::read_vec();
    let offset_u32 = sp1_zkvm::io::read::<u32>();
    let offset: usize = offset_u32.try_into().unwrap();

    let mut hash = [0u8; 32];
    let mut keccak256 = Keccak::v256();
    keccak256.update(&preimage);
    keccak256.finalize(&mut hash);

    let mut chunk = Vec::new();
    if offset < preimage.len() {
        let chunk_end = std::cmp::min(offset.saturating_add(32), preimage.len());
        chunk = preimage[offset..chunk_end].to_vec();
    }

    sp1_zkvm::io::commit(&hash);
    sp1_zkvm::io::commit(&offset_u32);
    sp1_zkvm::io::commit(&chunk);
}
