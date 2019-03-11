use crate::builders::{MachineRepresentationBuilder, TransitionTableBuilder};
use crate::common::StateTrait;
use crate::machine_representation::MachineRepresentation;
use crate::transition_table::TransitionTable;
use hashbrown::HashSet;

#[derive(Debug)]
pub struct GeneralMachineRepresentation<StateTy, TableTy>
where
    StateTy: StateTrait,
    TableTy: TransitionTable<StateTy, InputTy = char> + std::fmt::Debug,
{
    states: HashSet<StateTy>,
    starting_state: StateTy,
    accepting_state: StateTy,
    rejecting_state: StateTy,
    alphabet: HashSet<char>,
    transition_table: TableTy,
}

#[derive(Debug)]
pub enum RepresentationCreationError<StateTy, TableTy>
where
    StateTy: StateTrait,
    TableTy: TransitionTable<StateTy, InputTy = char> + std::fmt::Debug,
{
    StartingStateNotSpecified,
    AcceptStateNotSpecified,
    RejectStateNotSpecified,
    TransitionTableStateMismatch(HashSet<StateTy>),
    TransitionTableAlphabetMismatch(HashSet<char>),
    TableConstructionError(TableTy::ErrorTy),
}

impl<StateTy, TableTy> MachineRepresentation<StateTy>
    for GeneralMachineRepresentation<StateTy, TableTy>
where
    StateTy: StateTrait,
    TableTy: TransitionTable<StateTy, InputTy = char> + std::fmt::Debug,
{
    type InputTy = char;
    type OutputTy = TableTy::OutputTy;
    type TableTy = TableTy;
    type ErrorTy = RepresentationCreationError<StateTy, TableTy>;

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
        Builder::TableBuilder: TransitionTableBuilder<StateTy, InputTy = char>,
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

        let state_diff: HashSet<_> = b
            .transition_table_builder()
            .states()
            .difference(&b.states())
            .cloned()
            .collect();
        if !state_diff.is_empty() {
            return Err(RepresentationCreationError::TransitionTableStateMismatch(
                state_diff,
            ));
        }

        // Validate alphabet
        let alpha_diff: HashSet<_> = b
            .transition_table_builder()
            .alphabet()
            .difference(&b.alphabet())
            .cloned()
            .collect::<HashSet<_>>();
        if !alpha_diff.is_empty() {
            return Err(RepresentationCreationError::TransitionTableAlphabetMismatch(alpha_diff));
        }

        let transition_table = TableTy::from_builder(b.transition_table_builder())
            .map_err(RepresentationCreationError::TableConstructionError)?;

        Ok(GeneralMachineRepresentation {
            states: b.states().clone(),
            starting_state,
            accepting_state,
            rejecting_state,
            alphabet: b.alphabet().clone(),
            transition_table,
        })
    }
}
