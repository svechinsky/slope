use super::ConflictSet;
use transaction::simple_transaction::SimpleTransaction;
use vertex::simple_vertex::SimpleVertex;

pub struct SimpleConflictSet {
  vertices: Vec<SimpleVertex<SimpleTransaction>>,
  prefered_vertice: SimpleVertex<SimpleTransaction>,
}
