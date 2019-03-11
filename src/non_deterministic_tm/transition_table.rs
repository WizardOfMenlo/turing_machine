use crate::common::Action;
use hashbrown::{HashMap, HashSet};

use crate::builders::TransitionTableBuilder;
use crate::common::StateTrait;
use crate::transition_table::TransitionTable;
use crate::utils::Never;

#[derive(Debug, Clone, Default)]
pub struct NonDeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    transitions: HashMap<(StateTy, char), HashSet<Action<StateTy>>>,
}

impl<StateTy> TransitionTable<StateTy> for NonDeterministicTransitionTable<StateTy>
where
    StateTy: StateTrait,
{
    type InputTy = char;
    type OutputTy = HashSet<Action<StateTy>>;

    // Ideally I would use !, but it is experimental
    type ErrorTy = Never;

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
                // Add, initializing if not present
                transitions
                    .entry((state.clone(), c))
                    .or_insert_with(HashSet::new)
                    .insert(act);
            }
        }

        Ok(NonDeterministicTransitionTable { transitions })
    }
}
