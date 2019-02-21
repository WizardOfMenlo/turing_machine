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

use crate::machine_representation::Action;

pub trait TransitionTable<StateTy>
where
    StateTy: Eq + Hash,
{
    type InputType;
    type OutputType;

    fn apply_transition_table(
        &self,
        state: StateTy,
        input: Self::InputType,
    ) -> Option<Self::OutputType>;
}

/// The Transition table for a [`TuringMachine`](../trait.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct DeterministicTransitionTable<T>
where
    T: Debug + Clone + Default + Eq + Hash,
{
    transitions: HashMap<T, HashMap<char, Action<T>>>,
}

impl<T> TransitionTable<T> for DeterministicTransitionTable<T>
where
    T: Debug + Clone + Default + Eq + Hash,
{
    type InputType = char;
    type OutputType = Action<T>;

    fn apply_transition_table(&self, state: T, input_char: Self::InputType) -> Option<Action<T>> {
        self.transitions
            .get(&state)
            .and_then(|actions| actions.get(&input_char))
            .cloned()
    }
}
