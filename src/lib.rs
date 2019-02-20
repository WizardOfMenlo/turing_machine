pub mod machine_parser;
pub(crate) mod machine_representation;

use crate::machine_representation::{
    compressed::{Action, TmRepresentation},
    Motion,
};

use std::fmt;
use std::iter;

/// A builder struct for a [`TuringMachine`](trait.TuringMachine.html)  
/// # Usage
/// ```
/// use turing_machine::{
///     machine_representation::compressed::TmRepresentation,
///     TuringMachine, TuringMachineBuilder, DeterministicTuringMachine,
/// };
///
/// let tm : DeterministicTuringMachine = TuringMachineBuilder::new()
///                 .representation(TmRepresentation::default())
///                 .tape(Vec::new())
///                 .into();
/// ```
#[derive(Debug, Default)]
pub struct TuringMachineBuilder {
    representation: TmRepresentation,
    tape: Vec<char>,
}

impl TuringMachineBuilder {
    /// Create a empty `TuringMachineBuilder`.  
    /// This uses a empty starting tape, a machine with only two states, the accepting and rejecting state.  
    /// Furthermore the transition table will be empty, so reading every input (including the empty word) will reject.  
    pub fn new() -> Self {
        TuringMachineBuilder {
            representation: TmRepresentation::default(),
            tape: Vec::new(),
        }
    }

    /// Sets the tape of the builder.  
    pub fn tape<T: Into<Vec<char>>>(mut self, tape: T) -> Self {
        self.tape = tape.into();
        self
    }

    /// Sets the representation of the machine
    pub fn representation<R: Into<TmRepresentation>>(mut self, repr: R) -> Self {
        self.representation = repr.into();
        self
    }

    /// Validates that the representation and the tape are consistent.  
    /// This should be called on any conversion to avoid inconsistencies  
    /// Return `Some` if the builder is valid, `None` otherwise
    pub fn validate(self) -> Option<Self> {
        if self
            .tape
            .iter()
            .any(|tape_elem| !self.representation.alphabet().contains(tape_elem))
        {
            None
        } else {
            Some(self)
        }
    }
}

/// A trait encapsulating the behavior of a general TM
/// Should be general enough to account for [`DeterministicTuringMachine`](struct.DeterministicTuringMachine.html), k-tape machines, NDTMs and so on
pub trait TuringMachine: From<TuringMachineBuilder> {
    /// Each TM should be constructible from a Builder
    fn new<R>(repr: R) -> Self
    where
        R: Into<TuringMachineBuilder>;

    /// Takes a single step
    fn step(&mut self);

    /// Is the machine currently in an accepting state?
    fn is_accepting(&self) -> bool;

    /// Is the machine currently in a rejecting state?
    fn is_rejecting(&self) -> bool;

    /// Runs the TM until either an accepting or a rejecting state is reached.  
    /// Note this method might not return at all! Use with caution!
    fn run(&mut self) -> bool {
        while !(self.is_accepting() || self.is_rejecting()) {
            self.step();
        }
        self.is_accepting()
    }
}

/// Struct representing a TM with deterministic behaviour, singly infinite tape and variable alphabet  
/// This is (almost) the most basic TM that one can conceive.
#[derive(Debug)]
pub struct DeterministicTuringMachine {
    tape: Vec<char>,
    representation: TmRepresentation,
    current_cell: usize,
    current_state: usize,
    reject_state: usize,
}

impl DeterministicTuringMachine {
    fn apply_action(&mut self, act: &Action) {
        match act.motion() {
            Motion::Right => {
                if self.tape.len() <= self.current_cell {
                    let new_section = iter::repeat('_').take(self.tape.len() + 1);
                    self.tape.extend(new_section);
                }
                self.current_cell += 1;
            }
            Motion::Left => self.current_cell = self.current_cell.saturating_sub(1),
            Motion::Stay => {}
        };
        self.tape[self.current_cell] = *act.tape_output();
        self.current_state = *act.next_state();
    }
}

impl TuringMachine for DeterministicTuringMachine {
    fn new<R>(repr: R) -> Self
    where
        R: Into<TuringMachineBuilder>,
    {
        let builder = repr
            .into()
            .validate()
            .expect("Representation must be valid");
        let starting_state = *builder.representation.starting_state();
        let reject_state = *builder
            .representation
            .states()
            .iter()
            .filter(|(_, ty)| ty.is_rejecting())
            .map(|(n, _)| n)
            .next()
            .expect("Turing Machines with no reject state are illegal");

        DeterministicTuringMachine {
            tape: builder.tape,
            representation: builder.representation,
            current_cell: 0,
            current_state: starting_state,
            reject_state,
        }
    }

    fn step(&mut self) {
        if self.is_accepting() || self.is_rejecting() {
            return;
        }

        let input_char = self.tape[self.current_cell];
        let action = self
            .representation
            .transition_table()
            .get_action(self.current_state, input_char)
            .cloned()
            .unwrap_or_else(|| Action::new(self.reject_state, '_', Motion::Left));

        self.apply_action(&action);
    }

    fn is_accepting(&self) -> bool {
        let state = self.representation.states().get(&self.current_state);
        state.expect("Invalid State").is_accepting()
    }

    fn is_rejecting(&self) -> bool {
        let state = self.representation.states().get(&self.current_state);
        state.expect("Invalid State").is_rejecting()
    }
}

impl From<TuringMachineBuilder> for DeterministicTuringMachine {
    fn from(builder: TuringMachineBuilder) -> Self {
        DeterministicTuringMachine::new(builder)
    }
}

impl fmt::Display for DeterministicTuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Current State: {}", self.current_state)?;
        self.tape.iter().for_each(|v| write!(f, "{}", v).unwrap());
        Ok(())
    }
}
