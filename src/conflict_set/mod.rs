use vertex::Vertex;

use transaction::simple_transaction::SimpleTransaction;
pub mod simple_conflict_set;

trait ConflictSet<V:Vertex<SimpleTransaction>>{
  fn get_preferred(&self) -> V;
  fn get_vertices(&self) -> Vec<V>;
  fn add_conflicting_vertex(&mut self, V);

}