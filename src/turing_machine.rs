use crate::machine_representation::compressed::Action;
use crate::machine_representation::compressed::TmRepresentation;
use crate::machine_representation::Motion;
use crate::machine_representation::State;

use std::fmt;
use std::iter;

#[derive(Debug)]
pub struct TuringMachineBuilder {
    representation: TmRepresentation,
    tape: Vec<char>,
}

impl TuringMachineBuilder {
    pub fn new() -> Self {
        TuringMachineBuilder {
            representation: TmRepresentation::default(),
            tape: Vec::new(),
        }
    }

    pub fn tape<T: Into<Vec<char>>>(mut self, tape: T) -> Self {
        self.tape = tape.into();
        self
    }

    pub fn representation<R: Into<TmRepresentation>>(mut self, repr: R) -> Self {
        self.representation = repr.into();
        self
    }

    pub fn validate(self) -> Option<Self> {
        match self
            .tape
            .iter()
            .any(|tape_elem| !self.representation.alphabet().contains(tape_elem))
        {
            true => None,
            false => Some(self),
        }
    }
}

pub trait TuringMachine: From<TuringMachineBuilder> {
    fn new<R>(repr: R) -> Self
    where
        R: Into<TuringMachineBuilder>;

    fn step(&mut self);
    fn is_accepting(&self) -> bool;

    fn run(&mut self) -> bool {
        while !self.is_accepting() {
            self.step();
        }
        true
    }
}

#[derive(Debug)]
pub struct DeterministicTuringMachine {
    tape: Vec<char>,
    representation: TmRepresentation,
    current_cell: usize,
    current_state: usize,
}

impl DeterministicTuringMachine {
    fn apply_action(&mut self, act: &Action) {
        match act.motion() {
            Motion::Right => {
                if self.tape.len() <= self.current_cell {
                    let new_section = iter::repeat('_').take(self.tape.len() + 1);
                    self.tape.extend(new_section);
                }
                self.current_cell += 1;
            }
            Motion::Left => self.current_cell = self.current_cell.saturating_sub(1),
            Motion::Stay => {}
        };
        self.tape[self.current_cell] = *act.tape_output();
        self.current_state = *act.next_state();
    }
}

impl TuringMachine for DeterministicTuringMachine {
    fn new<R>(repr: R) -> Self
    where
        R: Into<TuringMachineBuilder>,
    {
        let builder = repr
            .into()
            .validate()
            .expect("Representation must be valid");
        let starting_state = *builder.representation.starting_state();
        DeterministicTuringMachine {
            tape: builder.tape,
            representation: builder.representation,
            current_cell: 0,
            current_state: starting_state,
        }
    }

    fn step(&mut self) {
        if self.is_accepting() {
            return;
        }

        let input_char = self.tape[self.current_cell];
        let action = self
            .representation
            .transition_table()
            .get_action(self.current_state, input_char)
            .cloned()
            .expect("Invalid State");

        self.apply_action(&action);
    }

    fn is_accepting(&self) -> bool {
        let state = self.representation.states().get(&self.current_state);
        match state.expect("Invalid State") {
            State::Accepting => true,
            State::Rejecting => false,
        }
    }
}

impl From<TuringMachineBuilder> for DeterministicTuringMachine {
    fn from(builder: TuringMachineBuilder) -> Self {
        DeterministicTuringMachine::new(builder)
    }
}

impl fmt::Display for DeterministicTuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current State: {}\n", self.current_state)?;
        self.tape.iter().for_each(|v| write!(f, "{}", v).unwrap());
        Ok(())
    }
}
