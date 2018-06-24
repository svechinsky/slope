use super::Vertex;
use transaction::simple_transaction::SimpleTransaction;
use transaction::Transaction;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleVertex<T: Transaction> {
  transaction: T,
  parent_vertices: Vec<SimpleVertex<T>>,
  child_vertices: Vec<SimpleVertex<T>>,
  chit: u8,
  confidence: u64,
}

impl Vertex<SimpleTransaction> for SimpleVertex<SimpleTransaction> {
  fn calculate_confidence(&self) -> u64 {
    let mut confidence = self.chit as u64;
    for child in &self.child_vertices {
      confidence = confidence + child.calculate_confidence();
    }
    return confidence;
  }
  fn generate(transaction: SimpleTransaction, parent_set: Vec<Self>) -> Self {
    return SimpleVertex {
      transaction: transaction,
      parent_vertices: parent_set,
      child_vertices: Vec::new(),
      chit: 1,
      confidence: 1,
    };
  }
}
