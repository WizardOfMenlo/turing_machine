use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::common::*;

/// The Transition table for a [`TuringMachine`](../trait.TuringMachine.html)
#[derive(Debug, Clone, Default)]
pub struct TransitionTable<T>
where
    T: Debug + Clone + Default + Eq + Hash,
{
    transitions: HashMap<T, HashMap<char, Action<T>>>,
}

impl<T> TransitionTable<T>
where
    T: Debug + Clone + Default + Eq + Hash,
{
    /// Given a `state`, and the `input_char` on the tape, gets the [`Action<T>`](struct.Action.html)
    /// to apply next
    pub fn get_action(&self, state: T, input_char: char) -> Option<&Action<T>> {
        self.transitions
            .get(&state)
            .and_then(|actions| actions.get(&input_char))
    }
}

fn convert_to_char(s: &str) -> Option<char> {
    if s.len() != 1 {
        return None;
    }
    s.chars().next()
}

impl TransitionTable<String> {
    // Helper function for testing parser
    #[cfg(test)]
    pub(crate) fn transitions(&self) -> &HashMap<String, HashMap<char, Action<String>>> {
        &self.transitions
    }

    /// Given a iterator over some lines,  create a [`TransitionTable`](struct.TransitionTable.html)
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

            let action = Action::new(
                next_state.to_string(),
                convert_to_char(output_char)?,
                match motion_str {
                    "R" => Motion::Right,
                    "L" => Motion::Left,
                    "S" => Motion::Stay,
                    _ => return None,
                },
            );

            transitions
                .entry(start_state.to_string())
                .or_insert_with(HashMap::new)
                .insert(convert_to_char(input_char)?, action);
        }

        Some(TransitionTable { transitions })
    }
}

/// A representation for [`TuringMachineExt`](../trait.TuringMachineExt) to be built from.  
/// I think this can be used for DTM and NDTM pretty easily
#[derive(Debug, Clone)]
pub struct TmRepresentation<T: Debug + Clone + Default + Eq + Hash> {
    states: HashMap<T, State>,
    alphabet: HashSet<char>,
    transition_table: TransitionTable<T>,
    starting_state: T,
}

impl Default for TmRepresentation<usize> {
    fn default() -> Self {
        let mut default_states = HashMap::with_capacity(2);
        default_states.insert(0, State::Rejecting);
        default_states.insert(1, State::Accepting);

        let mut default_alpha = HashSet::with_capacity(1);
        default_alpha.insert('_');

        TmRepresentation {
            states: default_states,
            alphabet: default_alpha,
            transition_table: TransitionTable::default(),
            starting_state: 0,
        }
    }
}

impl<T> TmRepresentation<T>
where
    T: Debug + Clone + Default + Eq + Hash,
{
    pub(crate) fn new(
        num_states: usize,
        starting_state: T,
        states: HashMap<T, State>,
        alphabet: HashSet<char>,
        transition_table: TransitionTable<T>,
    ) -> Self {
        assert_eq!(num_states, states.len());
        TmRepresentation {
            states,
            alphabet,
            transition_table,
            starting_state,
        }
    }

    /// Get the list of states in the representation
    pub fn states(&self) -> &HashMap<T, State> {
        &self.states
    }

    /// Get the starting state
    pub fn starting_state(&self) -> &T {
        &self.starting_state
    }

    /// Get the alphabet
    pub fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    /// Get the transition table
    pub fn transition_table(&self) -> &TransitionTable<T> {
        &self.transition_table
    }
}

impl From<TmRepresentation<String>> for TmRepresentation<usize> {
    fn from(source: TmRepresentation<String>) -> Self {
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

        let orig_transitions = source.transition_table().transitions.clone();

        // Construct a transition table
        let mut transition_table = TransitionTable {
            transitions: HashMap::with_capacity(orig_transitions.len()),
        };

        // Iterate over the transition table
        for (state_id, actions_map) in orig_transitions {
            // Convert to correct state
            let corresponding_id = conversion.get(&state_id).expect("Invalid map provided");

            // Build a map for this state
            let mut corresponding_map = HashMap::with_capacity(actions_map.len());

            // For each  possible action in this tape
            for (tape_in, action) in actions_map {
                // Gather transition state
                let next_state = conversion
                    .get(action.next_state())
                    .expect("Invalid map provided");

                // Create a corresponding action
                let new_action = Action::new(*next_state, *action.tape_output(), *action.motion());
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
