use transaction::simple_transaction::SimpleTransaction;

struct SimpleVertex<T: Transaction>{
  transaction: T;
  parent_transactions: T;
}