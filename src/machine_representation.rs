use crate::builders::MachineRepresentationBuilder;
use crate::builders::TransitionTableBuilder;
use crate::transition_table::TransitionTable;
use std::fmt::Debug;

use hashbrown::HashSet;

use crate::common::StateTrait;

/// The representation of a Turing Machine. Note that, as from lecture, the only difference is in the Function (aka the [`TransitionTable`](../transition_table/trait.TransitionTable.html))  
pub trait MachineRepresentation<StateTy>: Sized
where
    StateTy: StateTrait,
{
    /// The `InputTy` of the underlying `TransitionTable`
    type InputTy;

    /// The `OutputTy` of the underlying `TransitionTable`
    type OutputTy;

    /// The underlying `TransitionTable`
    type TableTy: TransitionTable<StateTy, InputTy = Self::InputTy, OutputTy = Self::OutputTy>;

    /// The Error type raised on invalid construction
    type ErrorTy: Debug;

    /// Get the list of states in the representation
    fn states(&self) -> &HashSet<StateTy>;

    /// Get the starting state
    fn starting_state(&self) -> &StateTy;

    /// Get the accepting state
    fn accepting_state(&self) -> &StateTy;

    /// Get the rejecting state
    fn rejecting_state(&self) -> &StateTy;

    /// Get the alphabet
    fn alphabet(&self) -> &HashSet<char>;

    /// Get the transition table
    fn transition_table(&self) -> &Self::TableTy;

    /// Build the representation from a [`MachineRepresentationBuilder`](../builders/trait.MachineRepresentationBuilder.html)  
    /// Note that we ensure the types match
    fn from_builder<Builder>(b: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: MachineRepresentationBuilder<StateTy>,
        Builder::TableBuilder: TransitionTableBuilder<StateTy>;
}
