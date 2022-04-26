use core::char;
use std::fs;
use std::path::Path;

pub struct Rotor {
    chars: Vec<char>,
    turnover_position: Option<usize>,
    position: usize,
    turnover_has_occured: bool,
}

pub const NUMBER_LETTERS_IN_ALPHABET: usize = 26;
pub const ASCII_LETTER_A: usize = 65;
pub const PATH: &str = "src/rotors.txt";

impl Rotor {
    pub fn new(mapping: &str, turnover_char: char) -> Rotor {
        let chars: Vec<char> = Rotor::mapping_to_vector(mapping);
        let turnover_position = Self::find_turnover_position(&turnover_char, &chars);
        Rotor {
            chars,
            turnover_position,
            position: 0,
            turnover_has_occured: false,
        }
    }

    pub fn turnover_has_occured(&self)  -> bool {
        self.turnover_has_occured
    }

    fn find_turnover_position(turnover_char: &char, chars: &[char]) -> Option<usize> {
        for (i, val) in chars.iter().enumerate() {
            if val == turnover_char {
                return Some(i);
            }
        }
        if *turnover_char == '_' {
            return None;
        }
        panic!()
    }

    pub fn encode_char(&mut self, input: char) -> char {
        let position_in_alphabet = get_position_in_alphabet(input);
        self.chars[(position_in_alphabet + self.position) % NUMBER_LETTERS_IN_ALPHABET]
    }

    pub fn encode_char_reverse(&mut self, input: char) -> char {
        let position_in_alphabet = get_position_in_alphabet(input);
        self.chars[(position_in_alphabet - self.position) % NUMBER_LETTERS_IN_ALPHABET]
    }

    pub fn increment_position(&mut self) {
        self.position = (self.position + 1) % NUMBER_LETTERS_IN_ALPHABET;
        if let Some(pos) = self.turnover_position {
            if pos == self.position {
                self.turnover_has_occured = true
            }
        }
    }

    fn mapping_to_vector(mapping: &str) -> Vec<char> {
        let mut vec: Vec<char> = Vec::with_capacity(NUMBER_LETTERS_IN_ALPHABET);
        for character in mapping.chars() {
            vec.push(character);
        }
        assert_eq!(vec.len(), NUMBER_LETTERS_IN_ALPHABET);
        vec
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn reset_turnover_state(&mut self) {
        self.turnover_has_occured = false;
    }

    pub fn from_file(path: &Path, id: &str) -> Rotor {
        let contents = fs::read_to_string(path).unwrap();
        for line in contents.lines() {
            if line.starts_with('#') {
                continue;
            }
            let mut items = line.split(':');
            if items.next().unwrap() == id {
                let mapping = items.next().unwrap();
                let turnover_char = items.next().unwrap().chars().next().unwrap();
                return Rotor::new(mapping, turnover_char);
            }
        }
        panic!();
    }
}

fn get_position_in_alphabet(input: char) -> usize {
    let upper = input.to_uppercase().next().unwrap() as usize;
    let position_in_alphabet = upper - ASCII_LETTER_A;
    assert!(position_in_alphabet < NUMBER_LETTERS_IN_ALPHABET);
    position_in_alphabet
}

#[cfg(test)]
mod tests {
    use crate::rotor::*;

    #[test]
    fn are_mappings_valid() {
        let vec = vec!["I", "II", "III"];
        for id in vec {
            let _ = Rotor::from_file(get_rotor_path(), id);
        }
    }

    fn get_rotor_path() -> &'static Path {
        Path::new(PATH)
    }

    fn get_instance() -> Rotor {
        Rotor::from_file(get_rotor_path(), "I")
    }

    #[test]
    fn increment_position() {
        let mut rotor = get_instance();
        rotor.set_position(0);
        assert_eq!(rotor.turnover_has_occured, false);
        for _ in 0..NUMBER_LETTERS_IN_ALPHABET {
            rotor.increment_position();
            if rotor.turnover_has_occured {
                return
            }
        }
        panic!();
    }
}
