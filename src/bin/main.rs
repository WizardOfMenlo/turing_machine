use clap::{App, Arg};
use log::{debug, error, info};
use std::{
    fmt::Display,
    fs::File,
    io::{self, Read},
    process,
};

use turing_machine::{
    builders::TuringMachineBuilder,
    deterministic_tm::{DeterministicMachineRepresentation, DeterministicTuringMachine},
    limited::LimitedTuringMachineExt,
    machine_parser::{self, ParsingError},
    machine_representation::MachineRepresentation,
    non_deterministic_tm::{NonDeterministicMachineRepresentation, NonDeterministicTuringMachine},
    stats::ExecutionResult,
    TuringMachine,
};

#[derive(Debug)]
enum ErrorType<T>
where
    T: TuringMachine,
{
    IO(io::Error),
    Parsing(ParsingError),
    ReprCreation(<T::ReprTy as MachineRepresentation<T::StateTy>>::ErrorTy),
    MachineCreation(T::ErrorTy),
}

impl<T> From<io::Error> for ErrorType<T>
where
    T: TuringMachine,
{
    fn from(err: io::Error) -> Self {
        ErrorType::IO(err)
    }
}

impl<T> From<ParsingError> for ErrorType<T>
where
    T: TuringMachine,
{
    fn from(err: ParsingError) -> Self {
        ErrorType::Parsing(err)
    }
}

fn run<T, Repr>(
    repr_path: &str,
    tape: Vec<char>,
    limit: Option<usize>,
) -> Result<ExecutionResult<T>, ErrorType<T>>
where
    T: TuringMachine<StateTy = String, ReprTy = Repr>,
    Repr: MachineRepresentation<String, InputTy = char>,
{
    info!("Tape: {:?}", tape);

    // Open the repr file
    let repr_file = File::open(repr_path)?;

    debug!("Parsing {} ...", repr_path);
    // Parse to TM bc
    let repr_builder = machine_parser::parse(repr_file)?;

    debug!("Building Representation ...");
    // Build the representation
    let repr = Repr::from_builder(&repr_builder).map_err(ErrorType::ReprCreation)?;

    debug!("Creating Machine Builder ...");
    // Adjoin with the tape
    let builder = TuringMachineBuilder::new().repr(repr).tape(tape);

    debug!("Creating Machine ...");
    // Build the machine
    let machine = T::from_builder(builder).map_err(ErrorType::MachineCreation)?;

    // Decorate with stats extension
    let machine = match limit {
        Some(limit) => LimitedTuringMachineExt::new_with_limit(machine, limit),
        None => LimitedTuringMachineExt::new(machine),
    };

    debug!("Execution Start ...");
    // Run to completion
    Ok(machine.execute_and_get_result())
}

fn handle_and_get_exit_code<T: TuringMachine + Display>(
    res: Result<ExecutionResult<T>, ErrorType<T>>,
) -> i32 {
    match res {
        Ok(exe) => {
            info!(" Machine ran for {} steps", exe.num_steps);
            info!(" Final configuration: {:?}", exe.tape);

            let res = if exe.accepting {
                println!("accepted");
                0
            } else {
                println!("not accepted");
                1
            };
            println!("{}", exe.num_steps - 1);
            print!("{}", exe.tm);

            res
        }
        Err(ty) => match ty {
            ErrorType::Parsing(e) => {
                error!("Parsing({:?})", e);
                2
            }
            ErrorType::ReprCreation(e) => {
                error!("Repr({:?})", e);
                2
            }
            ErrorType::MachineCreation(e) => {
                error!("Machine({:?})", e);
                2
            }
            ErrorType::IO(_) => 3,
        },
    }
}

fn main() -> io::Result<()> {
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
        .arg(
            Arg::with_name("ndtm")
                .long("nondeterministic")
                .short("n")
                .help("Use a non deterministic TM"),
        )
        .arg(
            Arg::with_name("limit")
                .takes_value(true)
                .value_name("STEP_LIMIT")
                .short("l")
                .long("limit")
                .help("Limit the number of steps the machine is allowed to take"),
        )
        .arg(
            Arg::with_name("tapevalue")
                .short("T")
                .long("tape")
                .conflicts_with("tapefile")
                .takes_value(true)
                .value_name("TAPE")
                .help("Inline tape for testing"),
        )
        .get_matches();

    // Initialize the logger
    env_logger::init();

    // Path is required, so it must be this
    let repr_path = matches.value_of("repr").unwrap();
    let tape_file = matches.value_of("tapefile");

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
        None => match matches.value_of("tapevalue") {
            Some(s) => s.chars().collect(),
            None => Vec::new(),
        },
    };

    let limit = matches
        .value_of("limit")
        .map(|s| s.parse::<usize>().expect("Parsing the argument failed"));

    let exit_code = if !matches.is_present("ndtm") {
        let result = run::<
            DeterministicTuringMachine<String>,
            DeterministicMachineRepresentation<String>,
        >(repr_path, tape, limit);
        handle_and_get_exit_code(result)
    } else {
        let result = run::<
            NonDeterministicTuringMachine<String>,
            NonDeterministicMachineRepresentation<String>,
        >(repr_path, tape, limit);
        handle_and_get_exit_code(result)
    };

    // Handle remaining case
    match exit_code {
        2 => println!("input error"),
        3 => println!("IO Error"),
        _ => {}
    }

    process::exit(exit_code);
}
