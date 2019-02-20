use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use turing_machine::machine_parser::{self, ParsingError};
use turing_machine::{DeterministicTuringMachine, TuringMachine, TuringMachineBuilder};

#[derive(Debug)]
enum ErrorType {
    IOError(io::Error),
    ParsingError(ParsingError),
}

impl From<io::Error> for ErrorType {
    fn from(err: io::Error) -> Self {
        ErrorType::IOError(err)
    }
}

impl From<ParsingError> for ErrorType {
    fn from(err: ParsingError) -> Self {
        ErrorType::ParsingError(err)
    }
}

fn main() -> Result<(), ErrorType> {
    let matches = App::new("Turing Machine")
        .version("0.1")
        .author("Giacomo Fenzi <giacomofenzi@outlook.com>")
        .about("Simulate a Turing Machine")
        .arg(
            Arg::with_name("repr")
                .required(true)
                .short("r")
                .takes_value(true)
                .value_name("FILE")
                .help("The representation file to use"),
        )
        .arg(
            Arg::with_name("tapefile")
                .short("t")
                .takes_value(true)
                .required_unless("tapeinput")
                .conflicts_with("tapeinput")
                .value_name("TAPE_FILE")
                .help("A file containing the tape the machine should start on"),
        )
        .arg(
            Arg::with_name("tapeinput")
                .short("i")
                .takes_value(true)
                .required_unless("tapefile")
                .conflicts_with("tapefile")
                .value_name("TAPE_INPUT")
                .help("A String representing the tape the machine should start on"),
        )
        .get_matches();

    // Path is required, so it must be this
    let repr_path = matches.value_of("repr").unwrap();

    // One of the two branches must necessarily be true
    let tape: Vec<char> = if matches.is_present("tapeinput") {
        matches.value_of("tapeinput").unwrap().chars().collect()
    } else {
        let input_path = matches.value_of("tapefile").unwrap();
        let mut input_file = File::open(input_path)?;
        let mut buf = String::new();
        input_file.read_to_string(&mut buf)?;
        buf.chars().collect()
    };

    // Open the repr file
    let repr_file = File::open(repr_path)?;

    // Parse the machine
    let repr = machine_parser::parse(repr_file)?;
    let builder = TuringMachineBuilder::new().representation(repr).tape(tape);

    // Run to completion
    let mut machine: DeterministicTuringMachine = builder.into();
    machine.run();

    Ok(())
}
