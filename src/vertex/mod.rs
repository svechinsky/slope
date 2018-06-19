use transaction::Transaction

pub trait Node{
    type Transaction: Transaction;
    fn validate(&self, &T) -> bool; 
    
}
