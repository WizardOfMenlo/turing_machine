use super::transition_table::{DeterministicTransitionTable, TableCreationError};
use crate::builders::{MachineRepresentationBuilder, TransitionTableBuilder};
use crate::common::{Action, State};
use crate::machine_representation::MachineRepresentation;
use crate::transition_table::TransitionTable;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// TODO, check we can default construct properly
#[derive(Debug, Default)]
pub struct DeterministicMachineRepresentation<StateTy>
where
    StateTy: Debug + Hash + Eq + Clone + Default,
{
    states: HashMap<StateTy, State>,
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
    TableConstructionError(TableCreationError),
}

impl From<TableCreationError> for RepresentationCreationError {
    fn from(t: TableCreationError) -> Self {
        RepresentationCreationError::TableConstructionError(t)
    }
}

impl<StateTy> MachineRepresentation<StateTy> for DeterministicMachineRepresentation<StateTy>
where
    StateTy: Debug + Hash + Eq + Clone + Default,
{
    type InputTy = char;
    type OutputTy = Action<StateTy>;
    type TableTy = DeterministicTransitionTable<StateTy>;
    type ErrorTy = RepresentationCreationError;

    fn states(&self) -> &HashMap<StateTy, State> {
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
        Builder::TableBuilder:
            TransitionTableBuilder<StateTy, InputTy = Self::InputTy, OutputTy = Self::OutputTy>,
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

        Ok(Self {
            states: b.states().clone(),
            starting_state,
            accepting_state,
            rejecting_state,
            alphabet: b.alphabet().clone(),
            transition_table: DeterministicTransitionTable::from_builder(
                b.transition_table_builder(),
            )?,
        })
    }
}
