mod state;
mod transaction;

use state::simple_state::SimpleState;

fn main() {
    let state:SimpleState = SimpleState::new();
    println!("Hello, world!");
}
