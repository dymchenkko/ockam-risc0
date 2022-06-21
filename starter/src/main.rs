use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use signature_bls::SecretKey;
use  signature_bls::Signature;
use  signature_bls::PublicKey;
use rand::Rng;
use checker_core::Message;
use rand_core::{RngCore, OsRng};

fn main() {

    let data = Message{
        message: &rand::thread_rng().gen::<[u8; 32]>(),
    };

    let mut rand_generator = OsRng {};
    rand_generator.next_u32();

    let sk = SecretKey::random(rand_generator).unwrap();
    let signat = Signature::new(&sk, &data.message).unwrap();
    //let pk = PublicKey::from(&sk);
    //let nn = signat.verify(pk, data);

    let mut prover = Prover::new(&MULTIPLY_PATH, MULTIPLY_ID).unwrap();
    prover.add_input(to_vec(&signat).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&sk).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&data).unwrap().as_slice()).unwrap();
    let receipt = prover.run().unwrap();

   // let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    //println!("I know the factors of {}, and I can prove it!", c);

    receipt.verify(MULTIPLY_ID).unwrap();
}
