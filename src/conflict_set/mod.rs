use vertex::Vertex;
use transaction::Transaction;
use transaction::simple_transaction::SimpleTransaction;
pub mod simple_conflict_set;

trait ConflictSet<T:Transaction, V:Vertex<T>>{
  fn get_preferred(&self) -> &V;
  fn get_vertices(&self) -> &Vec<V>;
  fn add_conflicting_vertex(&mut self, V);

}