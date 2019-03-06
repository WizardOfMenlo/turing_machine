use crate::common::Action;
use std::collections::HashMap;

use crate::builders::TransitionTableBuilder;
use crate::common::StateTrait;
use crate::transition_table::TransitionTable;

/// The Transition table for a [`DeterministicTuringMachine`](../struct.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct DeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    transitions: HashMap<StateTy, HashMap<char, Action<StateTy>>>,
}

#[derive(Debug)]
pub enum TableCreationError {
    DuplicateInput,
    DuplicateState,
}

impl<StateTy> TransitionTable<StateTy> for DeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    type InputTy = char;
    type OutputTy = Action<StateTy>;
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
                if state_transitions.insert(c, act).is_some() {
                    // We want no duplicates
                    return Err(TableCreationError::DuplicateInput);
                }
            }
            if transitions.insert(state, state_transitions).is_some() {
                // Same with states
                return Err(TableCreationError::DuplicateState);
            }
        }

        Ok(DeterministicTransitionTable { transitions })
    }
}
