use super::transition_table::{DeterministicTransitionTable, TableCreationError};
use crate::builders::{MachineRepresentationBuilder, TransitionTableBuilder};
use crate::common::{Action, StateTrait};
use crate::machine_representation::MachineRepresentation;
use crate::transition_table::TransitionTable;
use std::collections::HashSet;

#[derive(Debug)]
pub struct DeterministicMachineRepresentation<StateTy>
where
    StateTy: StateTrait,
{
    states: HashSet<StateTy>,
    starting_state: StateTy,
    accepting_state: StateTy,
    rejecting_state: StateTy,
    alphabet: HashSet<char>,
    transition_table: DeterministicTransitionTable<StateTy>,
}

#[derive(Debug)]
pub enum RepresentationCreationError {
    StartingStateNotSpecified,
    AcceptStateNotSpecified,
    RejectStateNotSpecified,
    TransitionTableStateMismatch,
    TransitionTableAlphabetMismatch,
    TableConstructionError(TableCreationError),
}

impl From<TableCreationError> for RepresentationCreationError {
    fn from(t: TableCreationError) -> Self {
        RepresentationCreationError::TableConstructionError(t)
    }
}

impl<StateTy> MachineRepresentation<StateTy> for DeterministicMachineRepresentation<StateTy>
where
    StateTy: StateTrait,
{
    type InputTy = char;
    type OutputTy = Action<StateTy>;
    type TableTy = DeterministicTransitionTable<StateTy>;
    type ErrorTy = RepresentationCreationError;

    fn states(&self) -> &HashSet<StateTy> {
        &self.states
    }

    fn starting_state(&self) -> &StateTy {
        &self.starting_state
    }

    fn accepting_state(&self) -> &StateTy {
        &self.accepting_state
    }
    fn rejecting_state(&self) -> &StateTy {
        &self.rejecting_state
    }

    fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    fn transition_table(&self) -> &Self::TableTy {
        &self.transition_table
    }

    fn from_builder<Builder>(b: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: MachineRepresentationBuilder<StateTy>,
        Builder::TableBuilder: TransitionTableBuilder<StateTy>,
    {
        let starting_state = b
            .starting_state()
            .as_ref()
            .cloned()
            .ok_or(RepresentationCreationError::StartingStateNotSpecified)?;
        let accepting_state = b
            .accepting_state()
            .as_ref()
            .cloned()
            .ok_or(RepresentationCreationError::AcceptStateNotSpecified)?;
        let rejecting_state = b
            .rejecting_state()
            .as_ref()
            .cloned()
            .ok_or(RepresentationCreationError::RejectStateNotSpecified)?;

        // Validate states
        if !b
            .transition_table_builder()
            .states()
            .iter()
            .all(|s| b.states().contains(s))
        {
            return Err(RepresentationCreationError::TransitionTableStateMismatch);
        }

        // Validate alphabet
        if !b
            .transition_table_builder()
            .alphabet()
            .iter()
            .all(|c| b.alphabet().contains(c))
        {
            return Err(RepresentationCreationError::TransitionTableAlphabetMismatch);
        }

        let transition_table =
            DeterministicTransitionTable::from_builder(b.transition_table_builder())?;

        Ok(Self {
            states: b.states().clone(),
            starting_state,
            accepting_state,
            rejecting_state,
            alphabet: b.alphabet().clone(),
            transition_table,
        })
    }
}
