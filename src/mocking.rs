use crate::builders::*;
use crate::common::StateTrait;
use crate::machine_representation::MachineRepresentation;
use crate::transition_table::TransitionTable;
use crate::TuringMachine;

use hashbrown::HashSet;
use std::marker::PhantomData;

// A collection of implementations to help testing

/// All unit implementation live here, not calling any of the methods will cause a panic
mod unit_impls {

    use super::*;

    impl<T> TransitionTable<T> for ()
    where
        T: StateTrait,
    {
        type InputTy = ();
        type OutputTy = ();
        type ErrorTy = ();

        fn apply_transition_table(&self, _: &T, _: Self::InputTy) -> Option<Self::OutputTy> {
            unreachable!()
        }

        fn from_builder<Builder>(_: &Builder) -> Result<Self, Self::ErrorTy>
        where
            Builder: TransitionTableBuilder<T>,
        {
            unreachable!()
        }
    }

    impl<T> MachineRepresentation<T> for ()
    where
        T: StateTrait,
    {
        type InputTy = ();
        type OutputTy = ();
        type TableTy = ();
        type ErrorTy = ();

        fn states(&self) -> &HashSet<T> {
            unreachable!()
        }

        fn starting_state(&self) -> &T {
            unreachable!()
        }

        fn accepting_state(&self) -> &T {
            unreachable!()
        }

        fn rejecting_state(&self) -> &T {
            unreachable!()
        }

        fn alphabet(&self) -> &HashSet<()> {
            unreachable!()
        }

        fn transition_table(&self) -> &Self::TableTy {
            unreachable!()
        }

        fn from_builder<Builder>(_: &Builder) -> Result<Self, Self::ErrorTy>
        where
            Builder: MachineRepresentationBuilder<T>,
            Builder::TableBuilder: TransitionTableBuilder<T>,
        {
            unreachable!()
        }
    }

    impl TuringMachine for () {
        type StateTy = ();
        type Tape = ();
        type ReprTy = ();
        type ErrorTy = ();

        fn step(&mut self) {
            unreachable!()
        }

        fn is_accepting(&self) -> bool {
            unreachable!()
        }

        fn is_rejecting(&self) -> bool {
            unreachable!()
        }

        fn tape(&self) -> &Self::Tape {
            unreachable!()
        }

        fn from_builder(
            _: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
        ) -> Result<Self, Self::ErrorTy> {
            unreachable!()
        }
    }
}

/// Machine implementation that just does nothing (`run` will loop forever btw so don't use it)
pub(crate) struct MockMachine<T>
where
    T: StateTrait,
{
    pub(crate) p: PhantomData<T>,
    pub(crate) tape: Vec<char>,
}

impl<T> TuringMachine for MockMachine<T>
where
    T: StateTrait,
{
    type StateTy = T;
    type Tape = Vec<char>;
    type ReprTy = ();
    type ErrorTy = ();

    fn step(&mut self) {}

    fn is_accepting(&self) -> bool {
        false
    }

    fn is_rejecting(&self) -> bool {
        false
    }

    fn tape(&self) -> &Self::Tape {
        &self.tape
    }

    fn from_builder(
        _: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        Err(())
    }
}

pub(crate) struct MockRepr<T>
where
    T: StateTrait,
{
    pub(crate) alphabet: HashSet<char>,
    pub(crate) p: PhantomData<T>,
}

impl<T> MachineRepresentation<T> for MockRepr<T>
where
    T: StateTrait,
{
    type InputTy = char;
    type OutputTy = ();
    type TableTy = MockTable<T>;
    type ErrorTy = ();

    fn states(&self) -> &HashSet<T> {
        unreachable!()
    }

    fn starting_state(&self) -> &T {
        unreachable!()
    }

    fn accepting_state(&self) -> &T {
        unreachable!()
    }

    fn rejecting_state(&self) -> &T {
        unreachable!()
    }

    fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    fn transition_table(&self) -> &Self::TableTy {
        unreachable!()
    }

    fn from_builder<Builder>(_: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: MachineRepresentationBuilder<T>,
        Builder::TableBuilder: TransitionTableBuilder<T>,
    {
        unreachable!()
    }
}

pub(crate) struct MockTable<T>
where
    T: StateTrait,
{
    pub(crate) p: PhantomData<T>,
}

impl<T> TransitionTable<T> for MockTable<T>
where
    T: StateTrait,
{
    type InputTy = char;

    type OutputTy = ();

    type ErrorTy = ();

    fn apply_transition_table(&self, _: &T, _: Self::InputTy) -> Option<Self::OutputTy> {
        unreachable!()
    }

    /// Construct from a [`TransitionTableBuilder`](../builders/trait.TransitionTableBuilder.html)  
    /// Note that the `InputTy` and `OutputTy` need to match
    /// `Option<T>` in order to account for invalid parsing
    fn from_builder<Builder>(_: &Builder) -> Result<Self, Self::ErrorTy>
    where
        Builder: TransitionTableBuilder<T, InputTy = Self::InputTy>,
    {
        unreachable!()
    }
}
