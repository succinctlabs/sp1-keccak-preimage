//! A simple script to generate and verify the proof of a given program.

use rand::RngCore;
use sha3::{Digest, Keccak256};
use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let mut data = vec![0u8; 1024 * 1024];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut data);
    stdin.write_vec(data.clone());
    let hash: [u8; 32] = Keccak256::digest(&data).into();
    let offset = data.len() as u32 - 5;
    stdin.write(&offset);
    let expected_chunk = data[offset as usize..].to_vec();

    let start = std::time::Instant::now();
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    println!("Setup in {:?}", start.elapsed());
    let mut proof = client.prove_compressed(&pk, stdin).expect("proving failed");

    // Read output.
    assert_eq!(proof.public_values.read::<[u8; 32]>(), hash);
    assert_eq!(proof.public_values.read::<u32>(), offset);
    assert_eq!(proof.public_values.read::<Vec<u8>>(), expected_chunk);

    println!("Generated proof in {:?}", start.elapsed());

    // Verify proof.
    client
        .verify_compressed(&proof, &vk)
        .expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!(
        "successfully generated and verified proof for the program in {:?}!",
        start.elapsed()
    );
}
