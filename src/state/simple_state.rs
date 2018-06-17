extern crate ring;
extern crate untrusted;

use super::State;
use ::transaction::simple_transaction::SimpleTransaction;
use std::collections::HashMap;
use self::ring::{signature,rand};




pub struct SimpleState{
    // Public key to balance, simple :)
    state: HashMap<&'static [u8], u64>
}

impl SimpleState{
    pub fn new() -> SimpleState {
        SimpleState {
            state: HashMap::new()
        }
    }
}

impl State<&'static [u8], u64, SimpleTransaction> for SimpleState{
    fn get(&self, key:&'static [u8]) -> &u64{

        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes)).unwrap();
        let public = key_pair.public_key_bytes();
        const MESSAGE: &'static [u8] = b"hello, world";
        let sig = key_pair.sign(MESSAGE);

        return self.state.get(&key).unwrap_or(&0);
    }
    
    fn validate(&self, transaction: &SimpleTransaction) -> bool{
        return true;
    }
}