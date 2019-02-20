pub(crate) mod compressed;
pub(crate) mod expanded;

/// The set of movements that a [`TuringMachine`](../trait.TuringMachine.html) can take on a single transition
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Default for State {
    fn default() -> Self {
        State::Neutral
    }
}
