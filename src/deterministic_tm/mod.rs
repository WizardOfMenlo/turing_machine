pub mod transition_table;

use crate::{
    common::{Action, Motion, StateTrait},
    machine_representation::MachineRepresentation,
    transition_table::TransitionTable,
    utils::apply_action,
    TuringMachine, TuringMachineBuilder,
};

use crate::common::representation::GeneralMachineRepresentation;
use log::debug;
use transition_table::DeterministicTransitionTable;

pub type DeterministicMachineRepresentation<StateTy> =
    GeneralMachineRepresentation<StateTy, DeterministicTransitionTable<StateTy>>;

use std::fmt;

/// Struct representing a TM with deterministic behaviour, singly infinite tape and variable alphabet  
/// This is (almost) the most basic TM that one can conceive.
#[derive(Debug)]
pub struct DeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    tape: Vec<char>,
    representation: DeterministicMachineRepresentation<StateTy>,
    current_cell: usize,
    current_state: StateTy,
}

#[derive(Debug)]
pub enum MachineCreationError {
    TapeAlphabetMismatch,
}

impl<StateTy> TuringMachine for DeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    type Tape = Vec<char>;
    type StateTy = StateTy;
    type ReprTy = DeterministicMachineRepresentation<StateTy>;
    type ErrorTy = MachineCreationError;

    fn from_builder(
        builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        // TODO proper validation
        let (tape, repr) = builder
            .validate()
            .ok_or(MachineCreationError::TapeAlphabetMismatch)?
            .decompose();

        Ok(Self {
            tape,
            current_state: repr.starting_state().clone(),
            representation: repr,
            current_cell: 0,
        })
    }

    fn step(&mut self) {
        if self.is_accepting() || self.is_rejecting() {
            return;
        }

        let input_char = self.tape.get(self.current_cell).unwrap_or(&'_');

        debug!(
            "Read {} while in state {:?}",
            input_char, self.current_state
        );
        let action = self
            .representation
            .transition_table()
            .apply_transition_table(&self.current_state, *input_char)
            .unwrap_or_else(|| {
                Action::new(
                    self.representation.rejecting_state().clone(),
                    *input_char,
                    Motion::Left,
                )
            });

        apply_action(
            action,
            &mut self.tape,
            &mut self.current_cell,
            &mut self.current_state,
        );
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

impl<StateTy> fmt::Display for DeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .tape
            .iter()
            .collect::<String>()
            .trim_end_matches('_')
            .to_string();
        if s == "" {
            writeln!(f, "_")?;
            return Ok(());
        }
        writeln!(f, "{}", s)?;

        Ok(())
    }
}
