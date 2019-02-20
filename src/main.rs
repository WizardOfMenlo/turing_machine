mod machine_parser;
mod machine_representation;
mod turing_machine;

use crate::turing_machine::{DeterministicTuringMachine, TuringMachine, TuringMachineBuilder};
use std::fs::File;

fn main() {
    let f = File::open("test.txt").unwrap();
    let repr = machine_parser::parse(f);
    println!("{:?}", repr);
    let builder = TuringMachineBuilder::new()
        .representation(repr.unwrap())
        .tape(Vec::new());

    let mut machine: DeterministicTuringMachine = builder.into();
    machine.run();
}
