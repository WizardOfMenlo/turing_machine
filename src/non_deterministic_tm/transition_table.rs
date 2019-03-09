use crate::common::Action;
use std::collections::{HashMap, HashSet};

use crate::builders::TransitionTableBuilder;
use crate::common::StateTrait;
use crate::transition_table::TransitionTable;

#[derive(Debug, Clone, Default)]
pub struct NonDeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    transitions: HashMap<StateTy, HashMap<char, HashSet<Action<StateTy>>>>,
}

#[derive(Debug)]
pub enum TableCreationError {
    DuplicateState,
}

impl<StateTy> TransitionTable<StateTy> for NonDeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    type InputTy = char;
    type OutputTy = HashSet<Action<StateTy>>;
    type ErrorTy = TableCreationError;

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
        Builder: TransitionTableBuilder<StateTy>,
    {
        let mut transitions = HashMap::new();

        let states_it = b.states();
        for state in states_it {
            let associated_transitions = b.get_state_transitions(&state);
            let mut state_transitions = HashMap::new();
            for (c, act) in associated_transitions {
                // Add, initializing if not present
                state_transitions
                    .entry(c)
                    .or_insert_with(HashSet::new)
                    .insert(act);
            }
            if transitions.insert(state, state_transitions).is_some() {
                // No duplicate states are allowed
                return Err(TableCreationError::DuplicateState);
            }
        }

        Ok(NonDeterministicTransitionTable { transitions })
    }
}
