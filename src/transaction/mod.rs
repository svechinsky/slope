
pub mod simple_transaction;

pub trait Transaction{
    type GenerateTuple;
    fn validate_sender(&self) -> bool;
    fn get_message(&self) -> String;
    fn serialize(&self) -> String;
    fn deserialize(String) -> Self;
    fn generate(Self::GenerateTuple) -> Self;
}

