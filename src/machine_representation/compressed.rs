use super::{Motion, State};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct Action {
    next_state: usize,
    tape_output: char,
    motion: Motion,
}

impl Action {
    pub(crate) fn new(next_state: usize, tape_output: char, motion: Motion) -> Self {
        Action {
            next_state,
            tape_output,
            motion,
        }
    }

    pub fn next_state(&self) -> &usize {
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
    transitions: HashMap<usize, HashMap<char, Action>>,
}

impl TransitionTable {
    pub fn get_action(&self, state: usize, input_char: char) -> Option<&Action> {
        self.transitions
            .get(&state)
            .and_then(|actions| actions.get(&input_char))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TmRepresentation {
    states: HashMap<usize, State>,
    alphabet: HashSet<char>,
    transition_table: TransitionTable,
    starting_state: usize,
}

impl TmRepresentation {
    pub fn states(&self) -> &HashMap<usize, State> {
        &self.states
    }

    pub fn starting_state(&self) -> &usize {
        &self.starting_state
    }

    pub fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    pub fn transition_table(&self) -> &TransitionTable {
        &self.transition_table
    }
}

impl From<super::expanded::TmRepresentation> for TmRepresentation {
    fn from(source: super::expanded::TmRepresentation) -> Self {
        let states = source.states();
        let num_states = states.len();

        // Create a mapping String -> ID
        let mut conversion = HashMap::with_capacity(num_states);

        // State mapping
        let mut dest_states = HashMap::with_capacity(num_states);

        // Convert the states map, and store the mapping
        for (counter, (name, accepting)) in states.iter().enumerate() {
            conversion.insert(name.clone(), counter);
            dest_states.insert(counter, accepting.clone());
        }

        let orig_transitions = source.transition_table().transitions();

        // Construct a transition table
        let mut transition_table = TransitionTable {
            transitions: HashMap::with_capacity(orig_transitions.len()),
        };

        // Iterate over the transition table
        for (state_id, actions_map) in orig_transitions {
            // Convert to correct state
            let corresponding_id = conversion.get(state_id).expect("Invalid map provided");

            // Build a map for this state
            let mut corresponding_map = HashMap::with_capacity(actions_map.len());

            // For each  possible action in this tape
            for (&tape_in, action) in actions_map {
                // Gather transition state
                let next_state = conversion
                    .get(action.next_state())
                    .expect("Invalid map provided");

                // Create a corresponding action
                let new_action = Action {
                    next_state: *next_state,
                    tape_output: *action.tape_output(),
                    motion: *action.motion(),
                };
                corresponding_map.insert(tape_in, new_action);
            }
            transition_table
                .transitions
                .insert(*corresponding_id, corresponding_map);
        }

        TmRepresentation {
            states: dest_states,
            alphabet: source.alphabet().clone(),
            transition_table,
            starting_state: *conversion
                .get(source.starting_state())
                .expect("Starting state invalid"),
        }
    }
}
