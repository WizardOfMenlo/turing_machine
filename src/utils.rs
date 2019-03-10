use crate::common::*;
use std::iter;

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
