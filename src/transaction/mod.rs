
pub mod simple_transaction;

pub trait Transaction{
    fn validate_sender(&self) -> bool;
    fn get_message(&self) -> &str;
    fn serialize(&self) -> &str;
}

