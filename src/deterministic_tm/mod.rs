pub mod representation;
pub mod transition_table;

use crate::{
    common::{Action, Motion},
    machine_representation::MachineRepresentation,
    transition_table::TransitionTable,
    TuringMachine, TuringMachineBuilder,
};

use representation::DeterministicMachineRepresentation;

use std::{fmt, iter};

/// Struct representing a TM with deterministic behaviour, singly infinite tape and variable alphabet  
/// This is (almost) the most basic TM that one can conceive.
#[derive(Debug)]
pub struct DeterministicTuringMachine {
    tape: Vec<char>,
    representation: DeterministicMachineRepresentation<String>,
    current_cell: usize,
    current_state: String,
}

impl DeterministicTuringMachine {
    fn apply_action(&mut self, act: &Action<String>) {
        // Bound checks
        if self.current_cell + 1 >= self.tape.len() {
            let new_section = iter::repeat('_').take(self.tape.len() + 2);
            self.tape.extend(new_section);
        }

        match act.motion() {
            Motion::Right => self.current_cell += 1,
            Motion::Left => self.current_cell = self.current_cell.saturating_sub(1),
            Motion::Stay => {}
        };
        self.tape[self.current_cell] = *act.tape_output();
        self.current_state = act.next_state().clone();
    }
}

#[derive(Debug)]
pub enum MachineCreationError {
    TapeAlphabetMismatch,
}

impl TuringMachine for DeterministicTuringMachine {
    type Tape = Vec<char>;
    type StateTy = String;
    type ReprTy = DeterministicMachineRepresentation<String>;
    type ErrorTy = MachineCreationError;

    fn from_builder(
        builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        // TODO proper validation
        let (tape, repr) = builder
            .validate()
            .ok_or(MachineCreationError::TapeAlphabetMismatch)?
            .decompose();

        let starting_state = repr.starting_state().clone();

        Ok(Self {
            tape,
            representation: repr,
            current_cell: 0,
            current_state: starting_state,
        })
    }

    fn step(&mut self) {
        if self.is_accepting() || self.is_rejecting() {
            return;
        }

        let input_char = self.tape.get(self.current_cell).unwrap_or(&'_');
        let action = self
            .representation
            .transition_table()
            .apply_transition_table(&self.current_state, *input_char)
            .unwrap_or_else(|| {
                Action::new(
                    self.representation.rejecting_state().clone(),
                    '_',
                    Motion::Left,
                )
            });

        self.apply_action(&action);
    }

    fn tape(&self) -> &Self::Tape {
        &self.tape
    }

    fn is_accepting(&self) -> bool {
        &self.current_state == self.representation.accepting_state()
    }

    fn is_rejecting(&self) -> bool {
        &self.current_state == self.representation.rejecting_state()
    }
}

impl fmt::Display for DeterministicTuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Current State: {}", self.current_state)?;
        self.tape.iter().for_each(|v| write!(f, "{}", v).unwrap());
        Ok(())
    }
}
