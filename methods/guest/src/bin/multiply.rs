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
    let signature: [u8; 48] = a.signature.try_into().unwrap();
    let publickey: [u8; 96] = a.publickey.try_into().unwrap();

    let signature = Signature::from_bytes(&signature).unwrap();
    let publickey = PublicKey::from_bytes(&publickey).unwrap();
    let result = signature.verify(publickey, a.message);
    let result = result.unwrap_u8() == 1;
    env::commit(&result);
}