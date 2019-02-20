use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Read};

use crate::machine_representation::{
    expanded::{TmRepresentation, TransitionTable},
    State,
};

/// A Error type for errors returned by [`parse`](fn.parse.html).  
/// Each variant expresses a particular error type and can be used to diagnose format mistakes
#[derive(Debug)]
pub enum ParsingError {
    /// The states parsing failed
    StatesError,

    /// An integer field could not be converted
    IntParsing,

    /// The alphabet is inconsistent
    AlphabetError,

    /// The transition table could not be parsed
    TransitionTableError,

    /// Error encountered in interacting with `io`
    IOError(io::Error),
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IOError(err)
    }
}

/// Function used to parse a [`TmRepresentation`](../machine_representation/expanded/struct.TmRepresentation.html)  
/// Returns a `Result`, which on the `Err` case stores a [`ParsingError`](enum.ParsingError.html) detailing the error type
/// # Usage:
/// ```
/// # use std::io::Read;
/// use turing_machine::machine_parser::parse;
///
/// // Ideally this will be read from a file
/// let test_string = "states 3\ns0\nqa +\nqr -\nalphabet 1 a\ns0 a s0 a R\ns0 b qa b R\ns0 _ qr _ S\n";
/// let res = parse(test_string.as_bytes().by_ref());
/// assert!(res.is_ok());
/// ```
/// Invalid configuration:
/// ```
/// # use std::io::Read;
/// use turing_machine::machine_parser::parse;
///
/// let test_string = "some gibberish";
/// let res = parse(test_string.as_bytes().by_ref());
/// assert!(res.is_err());
/// ```
pub fn parse(source: impl Read) -> Result<TmRepresentation, ParsingError> {
    // Convert to a buffered reader
    let mut reader = BufReader::new(source);
    let mut current_line = String::new();

    // Read the states descr
    reader.read_line(&mut current_line)?;
    if !current_line.starts_with("states") {
        return Err(ParsingError::StatesError);
    }

    // Parse the states descr
    let states_token: Vec<&str> = current_line.split(' ').collect();
    if states_token.len() != 2 {
        return Err(ParsingError::StatesError);
    }

    let num_states = states_token[1]
        .trim()
        .parse::<usize>()
        .map_err(|_| ParsingError::IntParsing)?;

    // Parse each of the states
    let mut starting_state = None;
    let mut states_map = HashMap::with_capacity(num_states);
    for _ in 0..num_states {
        current_line.clear();
        reader.read_line(&mut current_line)?;

        let tokens: Vec<&str> = current_line.split(' ').collect();
        let num_tokens = tokens.len();
        if num_tokens == 0 || num_tokens > 2 {
            return Err(ParsingError::StatesError);
        }
        let state_name = tokens[0].trim();
        let acceptance = match tokens.get(1).map(|s| s.trim()) {
            Some("+") => State::Accepting,
            Some("-") => State::Rejecting,
            None => State::Neutral,
            _ => return Err(ParsingError::StatesError),
        };
        states_map.insert(state_name.to_string(), acceptance);
        starting_state.get_or_insert(state_name.to_string());
    }

    // If we haven't set the starting state, error out. Same if no accepting state or reject states
    if starting_state.is_none()
        || !states_map.values().any(|ty| ty.is_rejecting())
        || !states_map.values().any(|ty| ty.is_accepting())
    {
        return Err(ParsingError::StatesError);
    }

    // Clear the current line
    current_line.clear();

    // Read the alphabet descr
    reader.read_line(&mut current_line)?;
    if !current_line.starts_with("alphabet") {
        return Err(ParsingError::AlphabetError);
    }

    // Gather the number of elements
    let mut split_it = current_line.split(' ').skip(1);
    let num_alphabet_elements = split_it
        .next()
        .ok_or(ParsingError::AlphabetError)?
        .trim()
        .parse::<usize>()
        .map_err(|_| ParsingError::IntParsing)?;

    // Gather alphabet
    let mut alphabet = HashSet::with_capacity(num_alphabet_elements + 1);
    // Insert mandatory blank char
    alphabet.insert('_');

    for token in split_it {
        let token = token.trim();
        if token.len() != 1 {
            return Err(ParsingError::AlphabetError);
        }
        alphabet.insert(token.chars().next().unwrap());
    }

    // Sanity checks
    if alphabet.len() != num_alphabet_elements + 1 {
        return Err(ParsingError::AlphabetError);
    }

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(TmRepresentation::new(
        num_states,
        &starting_state.unwrap(),
        states_map,
        alphabet,
        TransitionTable::build_from_iter(lines.into_iter())
            .ok_or(ParsingError::TransitionTableError)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine_representation::Motion;
    use crate::machine_representation::State;

    #[test]
    fn valid_example1() {
        let test_string = "states 4\ns0\ns1\ns2 +\nqr -\nalphabet 2 a b\ns0 a s0 a R\ns0 b s1 b R\ns0 _ s2 _ S\ns1 b s1 b R\ns1 _ s2 _ S";
        let result = parse(test_string.as_bytes().by_ref());
        let representation = result.expect("The parse should have succeded");

        let states = representation.states();
        assert_eq!(states.len(), 4);

        let states_test_cases = [
            ("s0", State::Neutral),
            ("s1", State::Neutral),
            ("s2", State::Accepting),
            ("qr", State::Rejecting),
        ];
        for (e, a) in states_test_cases.iter() {
            assert_eq!(states.get(*e).expect("Key should be present"), a);
        }

        assert_eq!(representation.starting_state(), "s0");

        let alphabet = representation.alphabet();
        assert_eq!(alphabet.len(), 3);
        assert!(alphabet.contains(&'_'));
        assert!(alphabet.contains(&'a'));
        assert!(alphabet.contains(&'b'));

        let transitions = representation.transition_table().transitions();
        assert_eq!(transitions.len(), 2);
        let s0_actions = transitions.get("s0").expect("State not parsed");
        let s1_actions = transitions.get("s1").expect("State not parsed");
        assert_eq!(s0_actions.len(), 3);
        assert_eq!(s1_actions.len(), 2);

        let s0_actions_tests_cases = [
            ('a', "s0", 'a', Motion::Right),
            ('b', "s1", 'b', Motion::Right),
            ('_', "s2", '_', Motion::Stay),
        ];

        let s1_actions_tests_cases = [
            ('b', "s1", 'b', Motion::Right),
            ('_', "s2", '_', Motion::Stay),
        ];

        for (input_char, end_state, output_char, motion) in &s0_actions_tests_cases {
            let action = s0_actions.get(input_char).expect("Input char not handled");
            assert_eq!(action.next_state(), *end_state);
            assert_eq!(action.tape_output(), output_char);
            assert_eq!(action.motion(), motion);
        }

        for (input_char, end_state, output_char, motion) in &s1_actions_tests_cases {
            let action = s1_actions.get(input_char).expect("Input char not handled");
            assert_eq!(action.next_state(), *end_state);
            assert_eq!(action.tape_output(), output_char);
            assert_eq!(action.motion(), motion);
        }
    }

    #[test]
    fn valid_example2() {
        let test_string = "states 4\nq0\nq1\nqr -\nqa +\nalphabet 2 a b\nq0 a q0 a R\nq0 _ qr _ L\nq0 b q1 b R\nq1 a qr a L\nq1 b qr a L\nq1 _ qa b L";
        let result = parse(test_string.as_bytes().by_ref());
        let representation = result.expect("The parse should have succeded");

        let states = representation.states();
        assert_eq!(states.len(), 4);

        let states_test_cases = [
            ("q0", State::Neutral),
            ("q1", State::Neutral),
            ("qr", State::Rejecting),
            ("qa", State::Accepting),
        ];
        for (e, a) in states_test_cases.iter() {
            assert_eq!(states.get(*e).expect("Key should be present"), a);
        }

        assert_eq!(representation.starting_state(), "q0");

        let alphabet = representation.alphabet();
        assert_eq!(alphabet.len(), 3);
        assert!(alphabet.contains(&'_'));
        assert!(alphabet.contains(&'a'));
        assert!(alphabet.contains(&'b'));

        let transitions = representation.transition_table().transitions();
        assert_eq!(transitions.len(), 2);
        let q0_actions = transitions.get("q0").expect("State not parsed");
        let q1_actions = transitions.get("q1").expect("State not parsed");
        assert_eq!(q0_actions.len(), 3);
        assert_eq!(q1_actions.len(), 3);

        let q0_actions_tests_cases = [
            ('a', "q0", 'a', Motion::Right),
            ('b', "q1", 'b', Motion::Right),
            ('_', "qr", '_', Motion::Left),
        ];

        let q1_actions_tests_cases = [
            ('a', "qr", 'a', Motion::Left),
            ('b', "qr", 'a', Motion::Left),
            ('_', "qa", 'b', Motion::Left),
        ];

        for (input_char, end_state, output_char, motion) in &q0_actions_tests_cases {
            let action = q0_actions.get(input_char).expect("Input char not handled");
            assert_eq!(action.next_state(), *end_state);
            assert_eq!(action.tape_output(), output_char);
            assert_eq!(action.motion(), motion);
        }

        for (input_char, end_state, output_char, motion) in &q1_actions_tests_cases {
            let action = q1_actions.get(input_char).expect("Input char not handled");
            assert_eq!(action.next_state(), *end_state);
            assert_eq!(action.tape_output(), output_char);
            assert_eq!(action.motion(), motion);
        }
    }

    #[test]
    fn empty_repr() {
        let test_string = "";
        let result = parse(test_string.as_bytes().by_ref());
        assert!(result.is_err());
        if let ParsingError::StatesError = result.unwrap_err() {
        } else {
            panic!("Invalid Enum Variant");
        }
    }
}
