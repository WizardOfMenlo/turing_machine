use super::{Motion, State};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct Action {
    next_state: String,
    tape_output: char,
    motion: Motion,
}

impl Action {
    pub fn next_state(&self) -> &str {
        &self.next_state
    }

    pub fn tape_output(&self) -> &char {
        &self.tape_output
    }

    pub fn motion(&self) -> &Motion {
        &self.motion
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransitionTable {
    transitions: HashMap<String, HashMap<char, Action>>,
}

fn convert_to_char(s: &str) -> Option<char> {
    if s.len() != 1 {
        return None;
    }
    s.chars().next()
}

impl TransitionTable {
    pub fn transitions(&self) -> &HashMap<String, HashMap<char, Action>> {
        &self.transitions
    }

    pub fn build_from_iter<I: IntoIterator<Item = String>>(iter: I) -> Option<Self> {
        let mut transitions = HashMap::new();
        for line in iter {
            let tokens: Vec<&str> = line.split(' ').collect();
            if tokens.len() != 5 {
                return None;
            }

            let start_state = tokens[0].trim();
            let input_char = tokens[1].trim();
            let next_state = tokens[2].trim();
            let output_char = tokens[3].trim();
            let motion_str = tokens[4].trim();

            let action = Action {
                next_state: next_state.to_string(),
                tape_output: convert_to_char(output_char)?,
                motion: match motion_str {
                    "R" => Motion::Right,
                    "L" => Motion::Left,
                    "S" => Motion::Stay,
                    _ => return None,
                },
            };

            transitions
                .entry(start_state.to_string())
                .or_insert_with(HashMap::new)
                .insert(convert_to_char(input_char)?, action);
        }

        Some(TransitionTable { transitions })
    }
}

#[derive(Debug, Clone, Default)]
pub struct TmRepresentation {
    num_states: usize,
    starting_state: String,
    states: HashMap<String, State>,
    alphabet: HashSet<char>,
    transition_table: TransitionTable,
}

impl TmRepresentation {
    pub(crate) fn new<S, A, T>(
        num_states: usize,
        starting_state: &str,
        states: S,
        alphabet: A,
        transition_table: T,
    ) -> Self
    where
        S: Into<HashMap<String, State>>,
        A: Into<HashSet<char>>,
        T: Into<TransitionTable>,
    {
        Self {
            num_states,
            starting_state: starting_state.to_string(),
            states: states.into(),
            alphabet: alphabet.into(),
            transition_table: transition_table.into(),
        }
    }

    pub fn states(&self) -> &HashMap<String, State> {
        &self.states
    }

    pub fn starting_state(&self) -> &str {
        &self.starting_state
    }

    pub fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    pub fn transition_table(&self) -> &TransitionTable {
        &self.transition_table
    }
}
