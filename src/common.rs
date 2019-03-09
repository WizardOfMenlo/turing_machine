use std::fmt::Debug;
use std::hash::Hash;

pub trait StateTrait: Debug + Clone + Default + Eq + Hash {}
impl<T> StateTrait for T where T: Debug + Clone + Default + Eq + Hash {}

/// The set of movements that a [`TuringMachine`](../trait.TuringMachine.html) can take on a single transition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Motion {
    Left,
    Right,
    Stay,
}

impl Default for Motion {
    fn default() -> Self {
        Motion::Stay
    }
}

/// The various characteristics a state of a [`TuringMachine`](../trait.TuringMachine.html) can have
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Accepting,
    Neutral,
    Rejecting,
}

impl State {
    /// Is the state an accepting state?
    pub fn is_accepting(&self) -> bool {
        match self {
            State::Accepting => true,
            _ => false,
        }
    }

    /// Is the state a rejecting state?
    pub fn is_rejecting(&self) -> bool {
        match self {
            State::Rejecting => true,
            _ => false,
        }
    }
}

/// Encapsulate the possible actions that can be done on the tape on a single step
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action<T>
where
    T: StateTrait,
{
    next_state: T,
    tape_output: char,
    motion: Motion,
}

impl<T> Action<T>
where
    T: StateTrait,
{
    pub(crate) fn new(next_state: T, tape_output: char, motion: Motion) -> Self {
        Action {
            next_state,
            tape_output,
            motion,
        }
    }

    /// Get the next state to move to
    pub fn next_state(&self) -> &T {
        &self.next_state
    }

    /// Get the symbol to write to the tape
    pub fn tape_output(&self) -> &char {
        &self.tape_output
    }

    /// Get the direction of motion
    pub fn motion(&self) -> &Motion {
        &self.motion
    }
}
