use ::transaction::Transaction;
pub mod simple_state;

pub trait State<K, S, T: Transaction>{
    fn get(&self, key: K) -> &S;
    fn validate(&self, &T) -> bool; 

}

