#![no_main]
#![no_std]

use risc0_zkvm_guest::env;
use checker_core::Information;
use  signature_bls::Signature;
use  signature_bls::PublicKey;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the first number from the host
    let a: Information = env::read();
    //let signature: [u8; 48] = a.signature.try_into().unwrap();
    let publickey: [u8; 96] = a.publickey.try_into().unwrap();

   // let signature = Signature::from_bytes(&signature).unwrap();
    let publickey = PublicKey::from_bytes(&publickey).unwrap();
   // let nn = signature.verify(publickey, a.message);
    // Load the second number from the host
    //let b: u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    /*if a == 1 || b == 1 {
        panic!("Trivial factors")
    }*/
    // Compute the product
    //let c: u64 = a * b;
    // Commit it to the public journal
    env::commit(&a);
}