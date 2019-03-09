pub mod representation;
pub mod transition_table;

use crate::{
    common::{Action, Motion, StateTrait},
    machine_representation::MachineRepresentation,
    transition_table::TransitionTable,
    TuringMachine, TuringMachineBuilder,
};

use representation::NonDeterministicMachineRepresentation;
use std::collections::HashSet;
use std::iter;

#[derive(Debug)]
pub struct NonDeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    states: Vec<StateTy>,
    positions: Vec<usize>,
    tapes: Vec<Vec<char>>,
    representation: NonDeterministicMachineRepresentation<StateTy>,
}

#[derive(Debug)]
pub enum MachineCreationError {
    TapeAlphabetMismatch,
}

fn apply_action<StateTy>(
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

impl<StateTy> TuringMachine for NonDeterministicTuringMachine<StateTy>
where
    StateTy: StateTrait,
{
    type Tape = Vec<Vec<char>>;
    type StateTy = StateTy;
    type ReprTy = NonDeterministicMachineRepresentation<StateTy>;

    type ErrorTy = MachineCreationError;

    fn step(&mut self) {
        // Don't do work if not needed!
        if self.is_accepting() || self.is_rejecting() {
            return;
        }

        // Sanity check, and find how many paths to simulate
        assert_eq!(self.states.len(), self.tapes.len());
        assert_eq!(self.tapes.len(), self.positions.len());
        let num_paths = self.states.len();

        // The additional paths will be added here
        let mut new_paths = HashSet::new();
        for i in 0..num_paths {
            let state = &mut self.states[i];

            // Skip rejecting states
            if state == self.representation.rejecting_state() {
                continue;
            }

            let corresponding_position = &mut self.positions[i];
            let corresponding_tape = &mut self.tapes[i];

            let char_on_tape = corresponding_tape
                .get(*corresponding_position)
                .unwrap_or(&'_');

            let possible_actions = self
                .representation
                .transition_table()
                .apply_transition_table(state, *char_on_tape)
                .unwrap_or_else(HashSet::new);

            // If we cannot proceed, then we set the machine in a rejecting state for this path
            if possible_actions.is_empty() {
                *state = self.representation.rejecting_state().clone();
                continue;
            }

            let mut actions_it = possible_actions.into_iter();
            // We handle the first one differently, to reduce too many allocations
            let first_act = actions_it.next().unwrap();

            for act in actions_it {
                // Make copies
                let mut new_tape = corresponding_tape.clone();
                let mut new_position = *corresponding_position;
                let mut new_state = state.clone();

                apply_action(act, &mut new_tape, &mut new_position, &mut new_state);
                new_paths.insert((new_tape, new_position, new_state));
            }

            // We do the first one first, so we don't invalidate the variables
            apply_action(first_act, corresponding_tape, corresponding_position, state);
        }

        // Put all in the end
        self.states
            .extend(new_paths.iter().map(|(_, _, state)| state).cloned());
        self.positions
            .extend(new_paths.iter().map(|(_, pos, _)| pos));
        self.tapes
            .extend(new_paths.into_iter().map(|(tape, _, _)| tape))

        // Todo, check no duplicate states (this is just for more efficiency really)
    }

    fn is_accepting(&self) -> bool {
        self.states
            .iter()
            .any(|x| x == self.representation.accepting_state())
    }

    fn is_rejecting(&self) -> bool {
        self.states
            .iter()
            .all(|x| x == self.representation.rejecting_state())
    }

    fn tape(&self) -> &Self::Tape {
        &self.tapes
    }

    fn from_builder(
        builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        let (tape, repr) = builder
            .validate()
            .ok_or(MachineCreationError::TapeAlphabetMismatch)?
            .decompose();

        Ok(Self {
            states: vec![repr.starting_state().clone()],
            positions: vec![0],
            tapes: vec![tape],
            representation: repr,
        })
    }
}
