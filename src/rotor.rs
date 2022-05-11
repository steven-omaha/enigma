use crate::alphabet::{
    get_position_in_alphabet, ALPHABET, ASCII_LETTER_A, NUMBER_LETTERS_IN_ALPHABET,
};
use core::char;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::iter::zip;
use std::path::Path;
use std::str::Split;

pub const PATH: &str = "src/rotors.txt";

#[derive(PartialEq)]
enum ShiftDirection {
    Forward,
    Reverse,
}

pub struct Rotor {
    forward: HashMap<char, char>,
    reverse: HashMap<char, char>,
    mapping: Vec<char>,
    turnover_position: usize,
    position: usize,
    turnover_has_occured: bool,
}

pub struct Reflector {
    chars: Vec<char>,
}

impl Reflector {
    pub fn from_file(path: &Path, id: &str) -> Reflector {
        let items = get_items_from_file_for_id(path, id);
        Reflector {
            chars: mapping_to_vector(&items.0),
        }
    }
}

impl Debug for Rotor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let turnover = match self.turnover_has_occured {
            true => "T",
            false => "_",
        };
        f.write_fmt(format_args!(
            "{}/{} {}",
            self.position, self.turnover_position, turnover
        ))
    }
}

pub trait Encode {
    fn encode_char(&mut self, input: char) -> char;
}

impl Encode for Rotor {
    fn encode_char(&mut self, input: char) -> char {
        *self
            .forward
            .get(&self.shift_char_by_position(input, ShiftDirection::Forward))
            .unwrap()
    }
}

impl Encode for Reflector {
    fn encode_char(&mut self, input: char) -> char {
        let position_in_alphabet = get_position_in_alphabet(input);
        self.chars[position_in_alphabet]
    }
}

impl Rotor {
    pub fn new(mapping: &str, turnover_char: char) -> Rotor {
        let chars: Vec<char> = mapping_to_vector(mapping);
        let turnover_position = Self::find_turnover_position(&turnover_char);
        let forward = Self::generate_forward_map(&chars);
        let reverse = Self::generate_reverse_map(&chars);
        Rotor {
            forward,
            reverse,
            mapping: chars,
            turnover_position,
            position: 0,
            turnover_has_occured: false,
        }
    }

    #[allow(dead_code)]  // used in tests
    pub fn get_position(&self) -> usize {
        self.position
    }

    fn generate_forward_map(vec: &[char]) -> HashMap<char, char> {
        let mut result: HashMap<char, char> = HashMap::with_capacity(NUMBER_LETTERS_IN_ALPHABET);
        for (input, output) in zip(a_to_z(), vec) {
            result.insert(input, *output);
        }
        result
    }

    fn generate_reverse_map(vec: &[char]) -> HashMap<char, char> {
        let mut result = HashMap::with_capacity(NUMBER_LETTERS_IN_ALPHABET);
        for (i, output) in vec.iter().enumerate() {
            result.insert(*output, *ALPHABET.get(i).unwrap());
        }
        result
    }

    pub fn turnover_has_occured(&self) -> bool {
        self.turnover_has_occured
    }

    fn find_turnover_position(turnover_char: &char) -> usize {
        for (i, val) in ALPHABET.iter().enumerate() {
            if val == turnover_char {
                return i;
            }
        }
        panic!();
    }

    pub fn increment_position(&mut self) {
        self.position = (self.position + 1) % NUMBER_LETTERS_IN_ALPHABET;
        if self.position == self.turnover_position {
            self.turnover_has_occured = true
        }
    }

    pub fn set_position(&mut self, position: usize) {
        assert!(position < NUMBER_LETTERS_IN_ALPHABET);
        self.position = position;
    }

    fn shift_char_by_position(&self, input: char, direction: ShiftDirection) -> char {
        let mut ascii = input as usize - ASCII_LETTER_A;
        match direction {
            ShiftDirection::Forward => ascii += self.position,
            ShiftDirection::Reverse => ascii -= self.position,
        }
        ascii = ascii % NUMBER_LETTERS_IN_ALPHABET + ASCII_LETTER_A;
        ascii as u8 as char
    }

    pub fn reset_turnover_state(&mut self) {
        self.turnover_has_occured = false;
    }

    pub fn from_file(path: &Path, id: &str) -> Rotor {
        let items = get_items_from_file_for_id(path, id);
        let mapping = items.0;
        let turnover_char = items.1.chars().next().unwrap();
        Rotor::new(&mapping, turnover_char)
    }

    pub fn encode_char_reverse(&mut self, input: char) -> char {
        let mut mapping = self.mapping.clone();
        mapping.rotate_left(self.position);
        let mut position = None;
        for (i, value) in mapping.iter().enumerate() {
            if input == *value {
                position = Some(i);
                break;
            }
        }
        match position {
            Some(p) => (p + ASCII_LETTER_A) as u8 as char,
            None => panic!(),
        }
    }
}

fn get_items_from_file_for_id<'a>(path: &'a Path, id: &'a str) -> (String, String) {
    let contents = fs::read_to_string(path).unwrap();
    for line in contents.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut items = line.split(':');
        if items.next().unwrap() == id {
            return extract_data(items);
        } else {
            continue;
        };
    }
    panic!("rotor not found");
}

#[allow(clippy::vec_init_then_push)]
fn extract_data(mut items: Split<char>) -> (String, String) {
    let pattern = items.next().expect("Rotor pattern missing").to_owned();
    let turnover_char = items
        .next()
        .expect("Turnover char not found. Consider using`_` as placeholder")
        .to_owned();
    (pattern, turnover_char)
}

fn mapping_to_vector(mapping: &str) -> Vec<char> {
    let vec: Vec<char> = mapping.chars().collect();
    assert_eq!(vec.len(), NUMBER_LETTERS_IN_ALPHABET);
    vec
}

fn a_to_z() -> Vec<char> {
    let mut result = Vec::new();
    for value in ASCII_LETTER_A..ASCII_LETTER_A + NUMBER_LETTERS_IN_ALPHABET {
        result.push(value as u8 as char);
    }
    assert_eq!(*result.get(0).unwrap(), 'A');
    assert_eq!(*result.get(NUMBER_LETTERS_IN_ALPHABET - 1).unwrap(), 'Z');
    result
}

#[cfg(test)]
mod tests {
    use crate::rotor::*;

    #[test]
    fn are_mappings_valid() {
        let vec = vec!["I", "II", "III"];
        for id in vec {
            Rotor::from_file(get_rotor_path(), id);
        }
        Reflector::from_file(get_rotor_path(), "B");
    }

    fn get_rotor_path() -> &'static Path {
        Path::new(PATH)
    }

    fn get_reflector_rotor_instance() -> Reflector {
        Reflector::from_file(get_rotor_path(), "B")
    }

    fn get_cypher_rotor_instance() -> Rotor {
        Rotor::from_file(get_rotor_path(), "I")
    }

    #[test]
    #[allow(unused_assignments)] // new_position is indeed used
    fn increment_position() {
        let mut rotor = get_cypher_rotor_instance();
        rotor.set_position(0);
        let mut old_position = 0;
        let mut new_position = 0;
        assert!(!rotor.turnover_has_occured);
        for _ in 0..NUMBER_LETTERS_IN_ALPHABET {
            rotor.increment_position();
            new_position = rotor.position;
            assert_eq!(
                (old_position + 1) % NUMBER_LETTERS_IN_ALPHABET,
                new_position
            );
            old_position = new_position;
        }
        assert!(rotor.turnover_has_occured);
    }

    #[test]
    fn pass_turnover_char() {
        let mut rotor = get_cypher_rotor_instance();
        rotor.set_position(16); // letter Q
        rotor.increment_position(); // letter R
        assert!(rotor.turnover_has_occured())
    }

    #[test]
    fn reflector_is_reversible() {
        for input in a_to_z() {
            let mut reflector = get_reflector_rotor_instance();

            let cypher = reflector.encode_char(input);
            let output = reflector.encode_char(cypher);

            assert_ne!(input, cypher);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn cypher_rotor_is_reversible() {
        let mut rotor = get_cypher_rotor_instance();
        for input in ALPHABET {
            for pos in 0..NUMBER_LETTERS_IN_ALPHABET {
                rotor.set_position(pos);
                let cypher = rotor.encode_char(input);
                rotor.set_position(pos);
                let decoded = rotor.encode_char_reverse(cypher);
                assert_eq!(input, decoded);
            }
        }
    }

    #[test]
    fn test_shift_char_by_position_forward() {
        let mut rotor = get_cypher_rotor_instance();
        rotor.set_position(0);
        for i in 0..NUMBER_LETTERS_IN_ALPHABET {
            let output = rotor.shift_char_by_position('A', ShiftDirection::Forward);
            assert_eq!(&output, ALPHABET.get(i).unwrap());
            rotor.increment_position();
        }
        let output = rotor.shift_char_by_position('A', ShiftDirection::Forward);
        assert_eq!(output, 'A');
    }

    #[test]
    fn test_shift_char_by_position_reverse() {
        let mut rotor = get_cypher_rotor_instance();
        rotor.set_position(0);
        for i in 0..NUMBER_LETTERS_IN_ALPHABET {
            let input = ALPHABET.get(i).unwrap();
            let output = rotor.shift_char_by_position(*input, ShiftDirection::Reverse);
            assert_eq!(output, 'A');
            rotor.increment_position();
        }
        let output = rotor.shift_char_by_position('A', ShiftDirection::Reverse);
        assert_eq!(output, 'A');
    }
}
