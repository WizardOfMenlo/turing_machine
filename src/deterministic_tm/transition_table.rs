use crate::common::Action;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::builders::TransitionTableBuilder;
use crate::transition_table::TransitionTable;

/// The Transition table for a [`DeterministicTuringMachine`](../struct.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct DeterministicTransitionTable<StateTy>
where
    StateTy: Debug + Clone + Default + Eq + Hash,
{
    transitions: HashMap<StateTy, HashMap<char, Action<StateTy>>>,
}

#[derive(Debug)]
pub enum TableConstructionError {
    DuplicateInputError,
    DuplicateState,
}

impl<StateTy> TransitionTable<StateTy> for DeterministicTransitionTable<StateTy>
where
    StateTy: Debug + Clone + Default + Eq + Hash,
{
    type InputTy = char;
    type OutputTy = Action<StateTy>;
    type ErrorTy = TableConstructionError;

    fn apply_transition_table(
        &self,
        state: &StateTy,
        input_char: Self::InputTy,
    ) -> Option<Self::OutputTy> {
        self.transitions
            .get(state)
            .and_then(|actions| actions.get(&input_char))
            .cloned()
    }

    fn from_builder<Builder>(b: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder:
            TransitionTableBuilder<StateTy, InputTy = Self::InputTy, OutputTy = Self::OutputTy>,
    {
        let mut transitions = HashMap::new();

        let states_it = b.states();
        for state in states_it {
            let associated_transitions = b.get_state_transitions(&state);
            let mut state_transitions = HashMap::new();
            for (c, act) in associated_transitions {
                if state_transitions.insert(c, act).is_some() {
                    // We want no duplicates
                    return Err(TableConstructionError::DuplicateInputError);
                }
            }
            if transitions.insert(state, state_transitions).is_some() {
                // Same with states
                return Err(TableConstructionError::DuplicateState);
            }
        }

        Ok(DeterministicTransitionTable { transitions })
    }
}
