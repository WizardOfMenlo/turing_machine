use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
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

fn run(repr_path: &str, tape_file: Option<&str>) -> Result<(), ErrorType> {
    // One of the two branches must necessarily be true
    let tape: Vec<char> = match tape_file {
        Some(p) => {
            let mut input_file = File::open(p)?;
            let mut buf = String::new();
            input_file.read_to_string(&mut buf)?;
            buf.chars().collect()
        }
        None => Vec::new(),
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

fn get_correct_exit_code(res: Result<(), ErrorType>) -> i32 {
    match res {
        Ok(()) => 0,
        Err(ty) => match ty {
            ErrorType::ParsingError(_) => 2,
            ErrorType::IOError(_) => 3,
        },
    }
}

fn main() {
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
                .value_name("TAPE_FILE")
                .help("A file containing the tape the machine should start on"),
        )
        .get_matches();

    // Path is required, so it must be this
    let repr_path = matches.value_of("repr").unwrap();
    let result = run(repr_path, matches.value_of("tapefile"));
    let exit_code = get_correct_exit_code(result);

    match exit_code {
        0 => println!("Accepted"),
        1 => println!("Rejected"),
        2 => println!("input error"),
        3 => println!("IO Error"),
        _ => unreachable!(),
    }

    std::process::exit(exit_code);
}