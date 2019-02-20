pub(crate) mod compressed;
pub(crate) mod expanded;

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

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Accepting,
    Neutral,
    Rejecting,
}

impl State {
    pub fn is_accepting(&self) -> bool {
        match self {
            State::Accepting => true,
            _ => false,
        }
    }

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
