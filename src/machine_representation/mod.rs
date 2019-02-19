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
    Rejecting,
}

impl Default for State {
    fn default() -> Self {
        State::Rejecting
    }
}
