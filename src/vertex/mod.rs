use transaction::Transaction;
use std::marker::Sized;

pub mod simple_vertex;

pub trait Vertex<T: Transaction>{
    fn calculate_confidence(&self) -> u64;
    fn generate(transaction: T, potential_parent_set: Vec<Self>) -> Self where Self:Sized;
}
