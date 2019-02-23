use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Read};

use crate::builders::{MachineRepresentationBuilder, TransitionTableBuilder};
use crate::common::*;

use lazy_static::lazy_static;

// TODO, add recursive error messages, for fine grained configuration errors

/// A Error type for errors returned by [`parse`](fn.parse.html).  
/// Each variant expresses a particular error type and can be used to diagnose format mistakes
#[derive(Debug)]
pub enum ParsingError {
    /// The states parsing failed
    States(StateError),

    /// The alphabet is inconsistent
    Alphabet(AlphabetError),

    /// The transition table could not be parsed
    TransitionTable(TransitionTableError),

    /// Error encountered in interacting with `io`
    IO(io::Error),
}

#[derive(Debug)]
pub enum StateError {
    MissingStateHeader,
    InvalidStateHeader,
    HeaderIntParsing,
    InvalidStateLine(String),
    InvalidStateName(String),
    InvalidStateSymbol(String),
    MandatoryStatesNotSet,
}

#[derive(Debug)]
pub enum AlphabetError {
    MissingAlphabetHeader,
    InvalidAlphabetHeader,
    InvalidBlankSymbol,
    HeaderIntParsing,
    TokenNotAChar(String),
    InvalidNumberOfElements(usize),
}

#[derive(Debug)]
pub enum TransitionTableError {
    InvalidNumberOfTokens(usize),
    TokenNotAChar(String),
    InvalidMotion(String),
    IO(io::Error),
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IO(err)
    }
}

impl From<StateError> for ParsingError {
    fn from(err: StateError) -> Self {
        ParsingError::States(err)
    }
}

impl From<AlphabetError> for ParsingError {
    fn from(err: AlphabetError) -> Self {
        ParsingError::Alphabet(err)
    }
}

impl From<TransitionTableError> for ParsingError {
    fn from(err: TransitionTableError) -> Self {
        match err {
            TransitionTableError::IO(err) => ParsingError::IO(err),
            _ => ParsingError::TransitionTable(err),
        }
    }
}

impl From<io::Error> for TransitionTableError {
    fn from(err: io::Error) -> Self {
        TransitionTableError::IO(err)
    }
}

/// The most general transition function I could think of. Think of this as the "bytecode" of the transition functions, each level up the hierarchy refining it and checking it
#[derive(Default, Debug)]
pub struct MachineTableParser {
    transitions: HashMap<String, Vec<(char, Action<String>)>>,
}

fn convert_to_char(s: &str) -> Option<char> {
    if s.len() != 1 {
        return None;
    }
    s.chars().next()
}

impl TransitionTableBuilder<String> for MachineTableParser {
    type InputTy = char;
    type OutputTy = Action<String>;

    type ErrorType = TransitionTableError;

    fn parse_line(&mut self, line: &str) -> Result<(), TransitionTableError> {
        let tokens: Vec<&str> = line.split(' ').collect();
        let num_tokens = tokens.len();
        if num_tokens != 5 {
            return Err(TransitionTableError::InvalidNumberOfTokens(num_tokens));
        }

        let start_state = tokens[0].trim();
        let input_char = tokens[1].trim();
        let next_state = tokens[2].trim();
        let output_char = tokens[3].trim();
        let motion_str = tokens[4].trim();

        let action = Action::new(
            next_state.to_string(),
            convert_to_char(output_char)
                .ok_or_else(|| TransitionTableError::TokenNotAChar(output_char.to_string()))?,
            match motion_str {
                "R" => Motion::Right,
                "L" => Motion::Left,
                "S" => Motion::Stay,
                _ => return Err(TransitionTableError::InvalidMotion(motion_str.to_string())),
            },
        );

        self.transitions
            .entry(start_state.to_string())
            .or_insert_with(Vec::new)
            .push((
                convert_to_char(input_char)
                    .ok_or_else(|| TransitionTableError::TokenNotAChar(input_char.to_string()))?,
                action,
            ));
        Ok(())
    }

    fn states(&self) -> Vec<String> {
        self.transitions.keys().cloned().collect()
    }

    fn get_state_transitions(&self, state: &String) -> Vec<(char, Action<String>)> {
        self.transitions
            .get(state)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}

/// The most general form of the element of the TM, to be successively parsed upwards
#[derive(Default, Debug)]
pub struct MachineParser {
    starting_state: Option<String>,
    accept_state: Option<String>,
    reject_state: Option<String>,

    states: HashMap<String, State>,
    alphabet: HashSet<char>,
    table_builder: MachineTableParser,
}

impl MachineParser {
    fn has_accept_state(&self) -> bool {
        self.accept_state.is_some()
    }

    fn has_reject_state(&self) -> bool {
        self.reject_state.is_some()
    }

    fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }
}

impl MachineRepresentationBuilder<String> for MachineParser {
    type InputTy = char;
    type OutputTy = Action<String>;
    type TableBuilder = MachineTableParser;

    type ErrorTy = ParsingError;

    fn add_state(&mut self, state: String, value: State) -> Result<(), ParsingError> {
        // TODO Check duplicates
        self.states.insert(state.clone(), value.clone());
        match value {
            State::Accepting => self.accept_state = Some(state),
            State::Rejecting => self.reject_state = Some(state),
            _ => {}
        }
        Ok(())
    }

    fn add_starting_state(&mut self, state: String) -> Result<(), ParsingError> {
        // TODO check if already set
        self.starting_state = Some(state);
        Ok(())
    }

    fn add_alphabet_symbol(&mut self, symbol: char) -> Result<(), ParsingError> {
        // TODO check errors
        self.alphabet.insert(symbol);
        Ok(())
    }

    fn get_transition_builder(&mut self) -> &mut Self::TableBuilder {
        &mut self.table_builder
    }

    fn states(&self) -> &HashMap<String, State> {
        &self.states
    }

    fn starting_state(&self) -> &Option<String> {
        &self.starting_state
    }

    fn accepting_state(&self) -> &Option<String> {
        &self.accept_state
    }
    fn rejecting_state(&self) -> &Option<String> {
        &self.reject_state
    }

    fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }

    fn transition_table_builder(&self) -> &Self::TableBuilder {
        &self.table_builder
    }
}

lazy_static! {
    static ref INVALID_STATE_NAMES: HashSet<&'static str> = {
        let mut s = HashSet::with_capacity(3);
        s.insert("alphabet");
        s.insert("+");
        s.insert("-");
        s
    };
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
pub fn parse(source: impl Read) -> Result<MachineParser, ParsingError> {
    let mut repr_builder = MachineParser::default();

    // Convert to a buffered reader
    let mut reader = BufReader::new(source);
    let mut current_line = String::new();

    // Read the states descr
    reader.read_line(&mut current_line)?;
    if !current_line.starts_with("states") {
        return Err(ParsingError::States(StateError::MissingStateHeader));
    }

    // Parse the states descr
    let states_token: Vec<&str> = current_line.split(' ').collect();
    if states_token.len() != 2 {
        return Err(ParsingError::States(StateError::InvalidStateHeader));
    }

    let num_states = states_token[1]
        .trim()
        .parse::<usize>()
        .map_err(|_| StateError::HeaderIntParsing)?;

    // Parse each of the states
    let mut starting_state = None;
    for _ in 0..num_states {
        current_line.clear();
        reader.read_line(&mut current_line)?;

        let tokens: Vec<&str> = current_line.split(' ').collect();
        let num_tokens = tokens.len();
        if num_tokens == 0 || num_tokens > 2 {
            return Err(ParsingError::States(StateError::InvalidStateLine(
                current_line,
            )));
        }

        let state_name = tokens[0].trim();
        if INVALID_STATE_NAMES.contains(&state_name) {
            return Err(ParsingError::States(StateError::InvalidStateName(
                state_name.to_string(),
            )));
        }

        let acceptance = match tokens.get(1).map(|s| s.trim()) {
            Some("+") => State::Accepting,
            Some("-") => State::Rejecting,
            None => State::Neutral,
            Some(symb) => {
                return Err(ParsingError::States(StateError::InvalidStateSymbol(
                    symb.to_string(),
                )));
            }
        };
        repr_builder.add_state(state_name.to_string(), acceptance)?;
        starting_state.get_or_insert(state_name.to_string());
    }

    // If we haven't set the starting state, error out. Same if no accepting state or reject states
    if starting_state.is_none()
        || !repr_builder.has_reject_state()
        || !repr_builder.has_accept_state()
    {
        return Err(ParsingError::States(StateError::MandatoryStatesNotSet));
    }

    repr_builder.add_starting_state(starting_state.unwrap())?;

    // Clear the current line
    current_line.clear();

    // Read the alphabet descr
    reader.read_line(&mut current_line)?;
    if !current_line.starts_with("alphabet") {
        return Err(ParsingError::Alphabet(AlphabetError::MissingAlphabetHeader));
    }

    // Gather the number of elements
    let mut split_it = current_line.split(' ').skip(1);
    let num_alphabet_elements = split_it
        .next()
        .ok_or(AlphabetError::InvalidAlphabetHeader)?
        .trim()
        .parse::<usize>()
        .map_err(|_| AlphabetError::HeaderIntParsing)?;

    // Insert mandatory blank char
    repr_builder.add_alphabet_symbol('_')?;

    for token in split_it {
        let token = token.trim();
        if token.len() != 1 {
            return Err(ParsingError::Alphabet(AlphabetError::TokenNotAChar(
                token.to_string(),
            )));
        }
        let c = token.chars().next().unwrap();
        // _ is not valid by specs
        if c == '_' {
            return Err(ParsingError::Alphabet(AlphabetError::InvalidBlankSymbol));
        }
        repr_builder.add_alphabet_symbol(c)?;
    }

    // Sanity checks
    let num_elements = repr_builder.alphabet_len();
    if num_elements != num_alphabet_elements + 1 {
        return Err(ParsingError::Alphabet(
            AlphabetError::InvalidNumberOfElements(num_elements),
        ));
    }

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    repr_builder
        .get_transition_builder()
        .build_from_lines(lines.into_iter())?;

    Ok(repr_builder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Motion, State};

    fn build_map(
        transition_builder: &MachineTableParser,
    ) -> &HashMap<String, Vec<(char, Action<String>)>> {
        &transition_builder.transitions
    }

    fn are_vecs_equal(a: &[(char, Action<String>)], b: &[(char, Action<String>)]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for (c1, action1) in a {
            let mut found = false;
            for (c2, action2) in b {
                if c1 == c2 && action1 == action2 {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        return true;
    }

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

        assert_eq!(representation.starting_state().as_ref().unwrap(), "s0");

        let alphabet = representation.alphabet();
        assert_eq!(alphabet.len(), 3);
        assert!(alphabet.contains(&'_'));
        assert!(alphabet.contains(&'a'));
        assert!(alphabet.contains(&'b'));

        let transitions_builder = representation.transition_table_builder();
        assert_eq!(transitions_builder.states().len(), 2);

        let transitions = build_map(transitions_builder);

        let s0_actions = transitions.get("s0").expect("State not parsed");
        let s1_actions = transitions.get("s1").expect("State not parsed");
        assert_eq!(s0_actions.len(), 3);
        assert_eq!(s1_actions.len(), 2);

        let s0_actions_tests_cases = [
            ('a', Action::new("s0".to_string(), 'a', Motion::Right)),
            ('b', Action::new("s1".to_string(), 'b', Motion::Right)),
            ('_', Action::new("s2".to_string(), '_', Motion::Stay)),
        ];

        let s1_actions_tests_cases = [
            ('b', Action::new("s1".to_string(), 'b', Motion::Right)),
            ('_', Action::new("s2".to_string(), '_', Motion::Stay)),
        ];

        if !(are_vecs_equal(s0_actions, &s0_actions_tests_cases[..])
            && are_vecs_equal(s1_actions, &s1_actions_tests_cases[..]))
        {
            panic!("Comparision failed");
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

        assert_eq!(representation.starting_state().as_ref().unwrap(), "q0");

        let alphabet = representation.alphabet();
        assert_eq!(alphabet.len(), 3);
        assert!(alphabet.contains(&'_'));
        assert!(alphabet.contains(&'a'));
        assert!(alphabet.contains(&'b'));

        let transitions_builder = representation.transition_table_builder();
        assert_eq!(transitions_builder.states().len(), 2);

        let transitions = build_map(transitions_builder);
        let q0_actions = transitions.get("q0").expect("State not parsed");
        let q1_actions = transitions.get("q1").expect("State not parsed");
        assert_eq!(q0_actions.len(), 3);
        assert_eq!(q1_actions.len(), 3);

        let q0_actions_tests_cases = [
            ('a', Action::new("q0".to_string(), 'a', Motion::Right)),
            ('b', Action::new("q1".to_string(), 'b', Motion::Right)),
            ('_', Action::new("qr".to_string(), '_', Motion::Left)),
        ];

        let q1_actions_tests_cases = [
            ('a', Action::new("qr".to_string(), 'a', Motion::Left)),
            ('b', Action::new("qr".to_string(), 'a', Motion::Left)),
            ('_', Action::new("qa".to_string(), 'b', Motion::Left)),
        ];

        if !(are_vecs_equal(q0_actions, &q0_actions_tests_cases[..])
            && are_vecs_equal(q1_actions, &q1_actions_tests_cases[..]))
        {
            panic!("Comparision failed");
        }
    }

    #[test]
    fn empty_repr() {
        let test_string = "";
        let result = parse(test_string.as_bytes().by_ref());
        assert!(result.is_err());
        match result {
            Err(ParsingError::States(StateError::MissingStateHeader)) => {}
            _ => panic!("Invalid Enum Variant"),
        }
    }
}
