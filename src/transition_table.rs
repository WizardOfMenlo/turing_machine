//! The transition table is the most integral part of each TuringMachine, changing its type can effectively
//! completely change the behaviour of the function.
//! In particular:
//! TM: `S x T -> S x T x {L, R, S}`
//! k-TM: `S x T^k -> S x T^K x {L, R, S}^K
//! NDTM: ` S x T -> P(S x T x {L, R, S})`
//!
//! Furthermore, we add an `Option` to the return type to allow for shortand specifications

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::builders::TransitionTableBuilder;
use crate::common::Action;

/// The Transition table for a [`TuringMachine`](../trait.TuringMachine.html)
pub trait TransitionTable<StateTy>: Sized
where
    StateTy: Eq + Hash,
{
    type InputType;
    type OutputType;

    fn apply_transition_table(
        &self,
        state: &StateTy,
        input: Self::InputType,
    ) -> Option<Self::OutputType>;

    fn from_builder<Builder>(b: &Builder) -> Option<Self>
    where
        Builder: TransitionTableBuilder<
            StateTy,
            InputType = Self::InputType,
            OutputType = Self::OutputType,
        >;
}

/// The Transition table for a [`DeterministicTuringMachine`](../struct.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct DeterministicTransitionTable<StateTy>
where
    StateTy: Debug + Clone + Default + Eq + Hash,
{
    transitions: HashMap<StateTy, HashMap<char, Action<StateTy>>>,
}

impl<StateTy> TransitionTable<StateTy> for DeterministicTransitionTable<StateTy>
where
    StateTy: Debug + Clone + Default + Eq + Hash,
{
    type InputType = char;
    type OutputType = Action<StateTy>;

    fn apply_transition_table(
        &self,
        state: &StateTy,
        input_char: Self::InputType,
    ) -> Option<Self::OutputType> {
        self.transitions
            .get(state)
            .and_then(|actions| actions.get(&input_char))
            .cloned()
    }

    fn from_builder<Builder>(b: &Builder) -> Option<Self>
    where
        Builder: TransitionTableBuilder<
            StateTy,
            InputType = Self::InputType,
            OutputType = Self::OutputType,
        >,
    {
        let mut transitions = HashMap::new();

        let states_it = b.states();
        for state in states_it {
            let associated_transitions = b.get_state_transitions(&state);
            let mut state_transitions = HashMap::new();
            for (c, act) in associated_transitions {
                if state_transitions.insert(c, act).is_some() {
                    // We want no duplicates
                    return None;
                }
            }
            if transitions.insert(state, state_transitions).is_some() {
                // Same with states
                return None;
            }
        }

        Some(DeterministicTransitionTable { transitions })
    }
}
