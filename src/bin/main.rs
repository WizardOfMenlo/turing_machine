use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use turing_machine::builders::TuringMachineBuilder;
use turing_machine::machine_parser::{self, ParsingError};
use turing_machine::machine_representation::MachineRepresentation;
use turing_machine::{
    deterministic_tm::{
        representation::DeterministicMachineRepresentation,
        representation::RepresentationCreationError, DeterministicTuringMachine,
        MachineCreationError,
    },
    stats::{ExecutionResult, TuringMachineStatsExt},
    TuringMachine,
};

#[derive(Debug)]
enum ErrorType {
    IO(io::Error),
    Parsing(ParsingError),
    ReprCreation(RepresentationCreationError),
    MachineCreation(MachineCreationError),
}

impl From<io::Error> for ErrorType {
    fn from(err: io::Error) -> Self {
        ErrorType::IO(err)
    }
}

impl From<ParsingError> for ErrorType {
    fn from(err: ParsingError) -> Self {
        ErrorType::Parsing(err)
    }
}

impl From<RepresentationCreationError> for ErrorType {
    fn from(err: RepresentationCreationError) -> Self {
        ErrorType::ReprCreation(err)
    }
}

impl From<MachineCreationError> for ErrorType {
    fn from(err: MachineCreationError) -> Self {
        ErrorType::MachineCreation(err)
    }
}

fn run(
    repr_path: &str,
    tape_file: Option<&str>,
) -> Result<ExecutionResult<impl TuringMachine>, ErrorType> {
    // One of the two branches must necessarily be true
    let tape: Vec<char> = match tape_file {
        Some(p) => {
            let mut input_file = File::open(p)?;
            let mut buf = String::new();
            input_file.read_to_string(&mut buf)?;
            buf.chars()
                .filter(|c| c.is_ascii() && !c.is_whitespace())
                .collect()
        }
        None => Vec::new(),
    };

    // Open the repr file
    let repr_file = File::open(repr_path)?;

    // Parse to TM bc
    let repr_builder = machine_parser::parse(repr_file)?;

    // Build the representation
    let repr = DeterministicMachineRepresentation::from_builder(&repr_builder)?;

    // Adjoin with the tape
    let builder = TuringMachineBuilder::new().repr(repr).tape(tape);

    // Build the machine
    let machine = DeterministicTuringMachine::from_builder(builder)?;

    // Decorate with stats extension
    let mut machine = TuringMachineStatsExt::new(machine);

    // Run to completion
    Ok(machine.execute_and_get_result())
}

fn handle_and_get_exit_code<T: TuringMachine>(res: Result<ExecutionResult<T>, ErrorType>) -> i32 {
    match res {
        Ok(exe) => {
            println!(" Machine ran for {} steps", exe.num_steps);
            println!(" Final configuration: {:?}", exe.tape);
            if exe.accepting {
                0
            } else {
                1
            }
        }
        Err(ty) => match ty {
            ErrorType::Parsing(_) | ErrorType::ReprCreation(_) | ErrorType::MachineCreation(_) => 2,
            ErrorType::IO(_) => 3,
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
                .index(1)
                .takes_value(true)
                .value_name("FILE")
                .help("The representation file to use"),
        )
        .arg(
            Arg::with_name("tapefile")
                .short("t")
                .index(2)
                .takes_value(true)
                .value_name("TAPE_FILE")
                .help("A file containing the tape the machine should start on"),
        )
        .get_matches();

    // Path is required, so it must be this
    let repr_path = matches.value_of("repr").unwrap();

    let result = run(repr_path, matches.value_of("tapefile"));
    let exit_code = handle_and_get_exit_code(result);

    match exit_code {
        0 => println!("Accepted"),
        1 => println!("Rejected"),
        2 => println!("input error"),
        3 => println!("IO Error"),
        _ => unreachable!(),
    }

    std::process::exit(exit_code);
}
