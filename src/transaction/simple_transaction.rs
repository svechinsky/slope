extern crate data_encoding;
extern crate ring;
extern crate untrusted;

extern crate serde;
extern crate serde_json;

use self::data_encoding::HEXLOWER;
use self::ring::signature;
use super::Transaction;
use super::error::TransactionError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleTransaction {
    pub value: u64,
    pub timestamp: u64,
    #[serde(with = "address")]
    pub to: Vec<u8>,
    #[serde(with = "address")]
    pub from: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Transaction for SimpleTransaction {
    type GenerateTuple = (u64, u64, Vec<u8>, Vec<u8>);
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
        return serde_json::to_string(&self).unwrap();
    }

    fn deserialize(transaction_message: String) -> Result<SimpleTransaction, TransactionError> {
        let transaction: SimpleTransaction =
            serde_json::from_str(transaction_message.as_ref()).unwrap();
        if transaction.validate_sender() {
            return Ok(transaction);
        } else {
            return Err(TransactionError::InvalidSigner);
        }
    }

    fn generate((value, timestamp, to, key_doc): Self::GenerateTuple) -> Self {
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(
            key_doc.as_slice(),
        )).unwrap();
        let public_key = key_pair.public_key_bytes();
        let msg = format!(
            "{},{},{},{}",
            value,
            timestamp,
            HEXLOWER.encode(to.as_slice()),
            HEXLOWER.encode(public_key)
        );
        let sig = key_pair.sign(msg.as_bytes());
        return SimpleTransaction {
            value: value,
            timestamp: timestamp,
            to: to,
            from: public_key.iter().cloned().collect(),
            signature: sig.as_ref().iter().cloned().collect(),
        };
    }
}

mod address {
    use super::serde::{Deserialize, Deserializer, Serializer};
    use super::HEXLOWER;

    pub fn serialize<S>(address: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", &HEXLOWER.encode(address.as_slice()));
        serializer.serialize_str(&s)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        return Ok(HEXLOWER
            .decode(s.as_bytes())
            .unwrap()
            .iter()
            .cloned()
            .collect());
    }
}

#[cfg(test)]
mod tests {
    extern crate time;

    use self::ring::rand;
    use super::*;

    #[test]
    fn generate_and_validate_simple_transaction() {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let now = time::precise_time_ns();

        let transaction =
            SimpleTransaction::generate((777, now, b"0x1fsaefse".to_vec(), pkcs8_bytes.to_vec()));

        let serialized = transaction.serialize();
        let deserialized: SimpleTransaction = serde_json::from_str(&serialized).unwrap();

        println!("Transaction {}", serialized);
        assert!(deserialized.validate_sender())
    }
}
