use super::ConflictSet;
use transaction::simple_transaction::SimpleTransaction;
use vertex::simple_vertex::SimpleVertex;

pub struct SimpleConflictSet {
  vertices: Vec<SimpleVertex<SimpleTransaction>>,
  prefered_vertex: SimpleVertex<SimpleTransaction>,
}

impl ConflictSet<SimpleTransaction, SimpleVertex<SimpleTransaction>> for SimpleConflictSet{
  fn get_preferred(&self) -> &SimpleVertex<SimpleTransaction>
  {
    return &self.prefered_vertex;
  }
  fn get_vertices(&self) -> &Vec<SimpleVertex<SimpleTransaction>>{
    return &self.vertices;
  }
  fn add_conflicting_vertex(&mut self, vertex:SimpleVertex<SimpleTransaction>){
    self.vertices.push(vertex);
  }
}
