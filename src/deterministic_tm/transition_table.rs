use crate::common::Action;
use hashbrown::HashMap;

use crate::builders::TransitionTableBuilder;
use crate::common::StateTrait;
use crate::transition_table::TransitionTable;

/// The Transition table for a [`DeterministicTuringMachine`](../struct.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct DeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    transitions: HashMap<(StateTy, char), Action<StateTy>>,
}

#[derive(Debug)]
pub enum TableCreationError<StateTy>
where
    StateTy: std::fmt::Debug,
{
    DuplicateTransition((StateTy, char)),
}

impl<StateTy> TransitionTable<StateTy> for DeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    type InputTy = char;
    type OutputTy = Action<StateTy>;
    type ErrorTy = TableCreationError<StateTy>;

    fn apply_transition_table(
        &self,
        state: &StateTy,
        input_char: Self::InputTy,
    ) -> Option<Self::OutputTy> {
        self.transitions.get(&(state.clone(), input_char)).cloned()
    }

    fn from_builder<Builder>(b: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: TransitionTableBuilder<StateTy, InputTy = char>,
    {
        let mut transitions = HashMap::new();

        let states_it = b.states();
        for state in states_it {
            let associated_transitions = b.get_state_transitions(&state);
            for (c, act) in associated_transitions {
                let transition = (state.clone(), c);
                if transitions.insert(transition.clone(), act).is_some() {
                    return Err(TableCreationError::DuplicateTransition(transition));
                }
            }
        }

        Ok(DeterministicTransitionTable { transitions })
    }
}
