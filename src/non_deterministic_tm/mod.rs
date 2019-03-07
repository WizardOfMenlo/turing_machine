pub mod representation;
pub mod transition_table;

use crate::{
    common::{Action, Motion, StateTrait},
    machine_representation::MachineRepresentation,
    transition_table::TransitionTable,
    TuringMachine, TuringMachineBuilder,
};

use representation::NonDeterministicMachineRepresentation;
use std::collections::HashSet;

/// TODO, is there a way to flatten this better?

#[derive(Debug)]
pub struct NonDeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    states: Vec<(StateTy, usize)>,
    tapes: Vec<Vec<char>>,
    representation: NonDeterministicMachineRepresentation<StateTy>,
}

// TODO, find a way to currectly implement all
impl<StateTy> NonDeterministicTuringMachine<StateTy> where StateTy: StateTrait {}

#[derive(Debug)]
pub enum MachineCreationError {
    TapeAlphabetMismatch,
}

impl<StateTy> TuringMachine for NonDeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    type Tape = Vec<Vec<char>>;
    type StateTy = StateTy;
    type ReprTy = NonDeterministicMachineRepresentation<StateTy>;

    type ErrorTy = MachineCreationError;

    fn step(&mut self) {
        if self.is_accepting() || self.is_rejecting() {
            return;
        }

        for i in 0..self.states.len() {
            if self.states[i].0 == *self.representation.rejecting_state() {
                continue;
            }
        }
    }

    fn is_accepting(&self) -> bool {
        self.states
            .iter()
            .any(|(x, _)| x == self.representation.accepting_state())
    }

    fn is_rejecting(&self) -> bool {
        self.states
            .iter()
            .all(|(x, _)| x == self.representation.rejecting_state())
    }

    fn tape(&self) -> &Self::Tape {
        &self.tapes
    }

    fn from_builder(
        builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        Err(MachineCreationError::TapeAlphabetMismatch)
    }
}
