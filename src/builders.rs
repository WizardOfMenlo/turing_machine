use crate::common::*;
use crate::machine_representation::MachineRepresentation;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

// TODO, would it generally be worth to replace at least some of these with structs? Or do we need the full generality of the problem?

/// A trait to generally parse a transition table
pub trait TransitionTableBuilder<StateTy>
where
    StateTy: StateTrait,
{
    type ErrorType: From<std::io::Error>;

    fn parse_line(&mut self, line: &str) -> Result<(), Self::ErrorType>;
    fn build_from_lines(
        &mut self,
        lines: impl Iterator<Item = String>,
    ) -> Result<(), Self::ErrorType> {
        for line in lines {
            self.parse_line(&line)?;
        }
        Ok(())
    }

    fn build_from_reader(&mut self, reader: impl Read) -> Result<(), Self::ErrorType> {
        let r = BufReader::new(reader);
        self.build_from_lines(r.lines().filter_map(|r| r.ok()))
    }

    fn states(&self) -> Vec<StateTy>;

    /// Given a current state, what is the transitions that we can take?
    fn get_state_transitions(&self, state: &StateTy) -> Vec<(char, Action<StateTy>)>;
}

pub trait MachineRepresentationBuilder<StateTy>
where
    StateTy: StateTrait,
{
    type TableBuilder: TransitionTableBuilder<StateTy>;

    type ErrorTy;

    fn add_state(&mut self, state: StateTy, value: State) -> Result<(), Self::ErrorTy>;
    fn add_starting_state(&mut self, state: StateTy) -> Result<(), Self::ErrorTy>;
    fn add_alphabet_symbol(&mut self, symbol: char) -> Result<(), Self::ErrorTy>;
    fn get_transition_builder(&mut self) -> &mut Self::TableBuilder;

    /// Get the list of states in the representation
    fn states(&self) -> &HashSet<StateTy>;

    /// Get the starting state
    fn starting_state(&self) -> &Option<StateTy>;

    fn accepting_state(&self) -> &Option<StateTy>;
    fn rejecting_state(&self) -> &Option<StateTy>;

    /// Get the alphabet
    fn alphabet(&self) -> &HashSet<char>;

    /// Get the transition table
    fn transition_table_builder(&self) -> &Self::TableBuilder;
}

/// A builder struct for a [`TuringMachine`](trait.TuringMachine.html)  
#[derive(Debug, Default)]
pub struct TuringMachineBuilder<StateTy, ReprTy>
where
    StateTy: StateTrait,
    ReprTy: MachineRepresentation<StateTy>,
{
    tape: Vec<char>,
    repr: Option<ReprTy>,
    marker: std::marker::PhantomData<StateTy>,
}

impl<StateTy, ReprTy> TuringMachineBuilder<StateTy, ReprTy>
where
    StateTy: StateTrait,
    ReprTy: MachineRepresentation<StateTy>,
{
    /// Create a empty `TuringMachineBuilder`.  
    /// This uses a empty starting tape, a machine with only two states, the accepting and rejecting state.  
    /// Furthermore the transition table will be empty, so reading every input (including the empty word) will reject.
    pub fn new() -> Self {
        TuringMachineBuilder {
            tape: Vec::new(),
            repr: None,
            marker: Default::default(),
        }
    }

    /// Sets the tape of the builder.  
    pub fn tape(mut self, tape: Vec<char>) -> Self {
        self.tape = tape;
        self
    }

    /// Sets the representation of the machine
    pub fn repr(mut self, repr: ReprTy) -> Self {
        self.repr = Some(repr);
        self
    }

    /// Convert the builder in a format easy to consume
    pub fn decompose(self) -> (Vec<char>, ReprTy) {
        (self.tape, self.repr.unwrap())
    }

    /// Validates that the representation and the tape are consistent.  
    /// This should be called on any conversion to avoid inconsistencies  
    /// Return `Some` if the builder is valid, `None` otherwise
    pub fn validate(self) -> Option<Self> {
        if self
            .tape
            .iter()
            .any(|tape_elem| !self.repr.as_ref().unwrap().alphabet().contains(tape_elem))
        {
            None
        } else {
            Some(self)
        }
    }
}
