extern crate ring;
extern crate untrusted;

use super::Transaction;
use self::ring::signature;
use std::str;

pub struct SimpleTransaction{
    pub value:u64,
    pub nonce:u64,
    pub to: &'static [u8],
    pub from: &'static [u8],
    pub signature: signature::Signature,
}

impl Transaction for SimpleTransaction{
    fn get_message(&self) -> &str{
        return &format!("value:{},nonce:{},to:{},from:{}", self.value, self.nonce, str::from_utf8(&self.to).unwrap() , str::from_utf8(&self.from).unwrap())
    }
    fn validate_sender(&self) ->bool{
        let message = self.get_message();
        signature::verify(&signature::ED25519, untrusted::Input::from(self.from), untrusted::Input::from(message.as_bytes()), untrusted::Input::from(self.signature.as_ref()));
        return true;

    }
    fn serialize(&self) -> &str{
        return "as"
    }

}