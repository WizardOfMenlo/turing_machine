use crate::builders::MachineRepresentationBuilder;
use crate::builders::TransitionTableBuilder;
use crate::common::State;
use crate::transition_table::{DeterministicTransitionTable, TransitionTable};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub trait MachineRepresentation<StateTy>: Sized
where
    StateTy: Hash + Eq,
{
    type TableTy: TransitionTable<StateTy>;

    /// Get the list of states in the representation
    fn states(&self) -> &HashMap<StateTy, State>;

    /// Get the starting state
    fn starting_state(&self) -> &StateTy;

    fn accepting_state(&self) -> &StateTy;
    fn rejecting_state(&self) -> &StateTy;

    /// Get the alphabet
    fn alphabet(&self) -> &HashSet<char>;

    /// Get the transition table
    fn transition_table(&self) -> &Self::TableTy;

    fn from_builder<Builder>(b: &Builder) -> Option<Self>
    where
        Builder: MachineRepresentationBuilder<StateTy>,
        Builder::TableBuilder: TransitionTableBuilder<
            StateTy,
            InputType = <Self::TableTy as TransitionTable<StateTy>>::InputType,
            OutputType = <Self::TableTy as TransitionTable<StateTy>>::OutputType,
        >;
}

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

impl<StateTy> MachineRepresentation<StateTy> for DeterministicMachineRepresentation<StateTy>
where
    StateTy: Debug + Hash + Eq + Clone + Default,
{
    type TableTy = DeterministicTransitionTable<StateTy>;

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

    fn from_builder<Builder>(b: &Builder) -> Option<Self>
    where
        Builder: MachineRepresentationBuilder<StateTy>,
        Builder::TableBuilder: TransitionTableBuilder<
            StateTy,
            InputType = <Self::TableTy as TransitionTable<StateTy>>::InputType,
            OutputType = <Self::TableTy as TransitionTable<StateTy>>::OutputType,
        >,
    {
        let starting_state = b.starting_state().as_ref().cloned()?;
        let accepting_state = b.accepting_state().as_ref().cloned()?;
        let rejecting_state = b.rejecting_state().as_ref().cloned()?;

        Some(DeterministicMachineRepresentation {
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
