#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
mod conflict_set;
mod state;
mod transaction;
mod vertex;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    if let Some(_start) = matches.subcommand_matches("start") {
        println!("No node to start yet :(");
    }
}
