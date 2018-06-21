use transaction::Transaction;

pub mod simple_vertex;

pub trait Vertex<T: Transaction>{
    fn select_parents(potential_parent_set: Vec<T>, number_of_parents:u32) -> Vec<T>;
    fn generate_vertex(transaction: T, potential_parent_set: Vec<T>) -> Self;
    
}
