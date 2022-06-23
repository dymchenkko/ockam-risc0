use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use signature_bls::SecretKey;
use signature_bls::Signature;
use signature_bls::PublicKey;
use rand::Rng;
use checker_core::Information;
use rand_core::{RngCore, OsRng};
use std::error::Error;

fn main() {

    let mut rand_generator = OsRng {};
    rand_generator.next_u32();

    let message = rand::thread_rng().gen::<[u8; 32]>();

    let sk = SecretKey::random(rand_generator).unwrap();
    let signat = Signature::new(&sk, &message).unwrap();
    let pk = PublicKey::from(&sk);

    let data = Information{
        signature: &signat.to_bytes(),
        publickey: &pk.to_bytes(),
        message: &message,
    };

    let mut prover = Prover::new(&MULTIPLY_PATH, MULTIPLY_ID).unwrap();
    
    prover.add_input(to_vec(&data).unwrap().as_slice()).unwrap();
    let receipt = prover.run().unwrap();
    let result = &receipt.get_journal_vec().unwrap();
    let result: Result<bool, _> = from_slice(result);
    let result = result.unwrap();
    println!("The result is: {}", result);

    receipt.verify(MULTIPLY_ID).unwrap();
}