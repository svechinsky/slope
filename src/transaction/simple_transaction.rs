extern crate ring;
extern crate untrusted;
extern crate data_encoding;


use self::ring::signature;
use super::Transaction;
use self::data_encoding::HEXLOWER;
use std::str;

pub struct SimpleTransaction {
    pub value: u64,
    pub timestamp: u64,
    pub to: Vec<u8>,
    pub from: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Transaction for SimpleTransaction {
    fn get_message(&self) -> String {
        return format!(
            "{},{},{},{}",
            self.value,
            self.timestamp,
            HEXLOWER.encode(self.to.as_slice()),
            HEXLOWER.encode(self.from.as_slice())
        );
    }
    fn validate_sender(&self) -> bool {
        let message = self.get_message();
        let verfication_result = signature::verify(
            &signature::ED25519,
            untrusted::Input::from(self.from.as_slice()),
            untrusted::Input::from(message.as_bytes()),
            untrusted::Input::from(self.signature.as_slice()),
        );

        match verfication_result {
            Ok(_res) => return true,
            Err(_err) => return false,
        }
    }
    fn serialize(&self) -> String {
        let message = self.get_message();
        return format!(
            "{},{}",
            message,
            &HEXLOWER.encode(self.signature.as_slice())
        );
    }

    fn deserialize(transaction_message: String) -> SimpleTransaction {
        let split: Vec<&str> = transaction_message.split(',').collect();
        return SimpleTransaction {
            value: split[0].parse::<u64>().unwrap(),
            timestamp: split[1].parse::<u64>().unwrap(),
            to: HEXLOWER.decode(split[2].as_bytes()).unwrap().iter().cloned().collect(),
            from: HEXLOWER.decode(split[3].as_bytes()).unwrap().iter().cloned().collect(),
            signature: HEXLOWER.decode(split[4].as_bytes()).unwrap().iter().cloned().collect(),
        };
    }
}

impl SimpleTransaction {
    pub fn generate(_value: u64, _timestamp: u64, _to: Vec<u8>, _key_doc: Vec<u8>) -> Self {
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(
            _key_doc.as_slice()
        )).unwrap();
        let public_key = key_pair.public_key_bytes();
        let msg = format!(
            "{},{},{},{}",
            _value,
            _timestamp,
            HEXLOWER.encode(_to.as_slice()),
            HEXLOWER.encode(public_key)
        );
        let sig = key_pair.sign(msg.as_bytes());
        return SimpleTransaction{value: _value, timestamp: _timestamp, to: _to, from: public_key.iter().cloned().collect(), signature: sig.as_ref().iter().cloned().collect()};
    }
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;
    use self::ring::rand;

    #[test]
    fn generate_and_validate_simple_transaction() {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let now = time::precise_time_ns();
        
        let transaction = SimpleTransaction::generate(777, now, b"0x1fsaefse".to_vec(), pkcs8_bytes.to_vec());
        println!("Transaction {}", transaction.serialize());
        assert!(transaction.validate_sender())
    }
}
