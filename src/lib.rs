pub mod builders;
pub mod common;
pub mod deterministic_tm;
pub mod machine_parser;
pub mod machine_representation;
pub mod stats;
pub mod transition_table;

use crate::builders::TuringMachineBuilder;
use crate::machine_representation::MachineRepresentation;
use std::hash::Hash;

/// A trait encapsulating the behavior of a general TM
/// Should be general enough to account for [`DeterministicTuringMachine`](struct.DeterministicTuringMachine.html), k-tape machines, NDTMs and so on
pub trait TuringMachine: Sized {
    /// Associated tape
    type Tape: Clone;
    type StateTy: Hash + Eq;
    type ReprTy: MachineRepresentation<Self::StateTy>;

    /// Takes a single step
    fn step(&mut self);

    /// Is the machine currently in an accepting state?
    fn is_accepting(&self) -> bool;

    /// Is the machine currently in a rejecting state?
    fn is_rejecting(&self) -> bool;

    /// Get the tape the machine is currently using
    fn tape(&self) -> &Self::Tape;

    /// Runs the TM until either an accepting or a rejecting state is reached.  
    /// Note this method might not return at all! Use with caution!
    fn run(&mut self) -> bool {
        while !(self.is_accepting() || self.is_rejecting()) {
            self.step();
        }
        self.is_accepting()
    }

    fn from_builder(builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>) -> Option<Self>;
}
