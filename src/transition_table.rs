//! The transition table is the most integral part of each TuringMachine, changing its type can effectively
//! completely change the behaviour of the function.
//! In particular:
//! TM: `S x T -> S x T x {L, R, S}`
//! k-TM: `S x T^k -> S x T^K x {L, R, S}^K`
//! NDTM: ` S x T -> P(S x T x {L, R, S})`
//!
//! Furthermore, we add an `Option` to the return type to allow for shortand specifications

use std::fmt::Debug;

use crate::{builders::TransitionTableBuilder, common::StateTrait};

/// Trait Encapsulating a Transition table for a [`TuringMachine`](../trait.TuringMachine.html)  
/// Functionally speaking, it represents: `F: (StateTy x InputTy) -> OutputTy`
pub trait TransitionTable<StateTy>: Sized
where
    StateTy: StateTrait,
{
    /// The input it reads from tape, generally a `char`
    type InputTy;

    /// The action the TM should take next, generally a [`Action<T>`](../common/struct.Action.html)
    type OutputTy;

    /// The error to be raise on invalid build
    type ErrorTy: Debug;

    /// Computes `F(state, input)`. Returns an `Option<T>` to account for invalid values and/or default options
    fn apply_transition_table(
        &self,
        state: &StateTy,
        input: Self::InputTy,
    ) -> Option<Self::OutputTy>;

    /// Construct from a [`TransitionTableBuilder`](../builders/trait.TransitionTableBuilder.html)  
    /// Note that the `InputTy` and `OutputTy` need to match
    /// `Option<T>` in order to account for invalid parsing
    fn from_builder<Builder>(b: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: TransitionTableBuilder<StateTy, InputTy = Self::InputTy>;
}
