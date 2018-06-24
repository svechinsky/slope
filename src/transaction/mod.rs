
pub mod simple_transaction;
pub mod error;
use std::marker::Sized;

pub trait Transaction {
    type GenerateTuple;
    fn validate_sender(&self) -> bool;
    fn get_message(&self) -> String;
    fn generate(Self::GenerateTuple) -> Self;
    fn serialize(&self) -> String;
    fn deserialize(String) -> Result<Self, error::TransactionError> where Self: Sized;
}
