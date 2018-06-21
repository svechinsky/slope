use transaction::Transaction;
use transaction::simple_transaction::SimpleTransaction;

pub struct SimpleVertex<T: Transaction>{
  transaction: T,
  parent_transactions: Vec<T>,
  chit: u8,
  confidence: u64
}