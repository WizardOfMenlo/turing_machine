use std::iter;

use crate::common::{Action, Motion, StateTrait};

/// Type that cannot be instantiated (like `!`)
#[derive(Debug)]
pub enum Never {}

pub fn apply_action<StateTy>(
    act: Action<StateTy>,
    tape: &mut Vec<char>,
    position: &mut usize,
    state: &mut StateTy,
) where
    StateTy: StateTrait,
{
    // Bound checks
    if *position + 1 >= tape.len() {
        let new_section = iter::repeat('_').take(tape.len() + 2);
        tape.reserve(tape.len() + 2);
        tape.extend(new_section);
    }

    // Write to cell
    tape[*position] = *act.tape_output();

    // New position
    match act.motion() {
        Motion::Right => *position += 1,
        Motion::Left => *position = position.saturating_sub(1),
        Motion::Stay => {}
    };

    // New state
    *state = act.next_state().clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_vector() {
        let mut v = Vec::new();
        let mut s = 0;
        let mut pos = 0;
        let act = Action::new(1, '1', Motion::Right);
        apply_action(act, &mut v, &mut pos, &mut s);
        assert_eq!(v[0], '1');
        assert_eq!(s, 1);
        assert_eq!(pos, 1);
    }
}
