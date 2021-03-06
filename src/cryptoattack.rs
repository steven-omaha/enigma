use crate::alphabet::NUMBER_LETTERS_IN_ALPHABET;
use crate::message::Message;
use crate::rotorassembly::RotorAssembly;
use crate::{rotor, Enigma, Plugboard};
use std::iter::zip;
use std::path::Path;

struct EnigmaAttack {
    enigma: Enigma,
    positions: [usize; 3],
}

impl EnigmaAttack {
    fn new(enigma: Enigma, positions: [usize; 3]) -> EnigmaAttack {
        for pos in &positions {
            assert!(pos < &NUMBER_LETTERS_IN_ALPHABET);
        }
        EnigmaAttack { enigma, positions }
    }

    fn new_default() -> EnigmaAttack {
        // order of rotors is correct, positions are not
        let plugboard = Plugboard::new(vec![]);
        let rotor_path = Path::new(rotor::PATH);
        let rotors = vec![
            rotor::Rotor::from_file(rotor_path, "I"),
            rotor::Rotor::from_file(rotor_path, "II"),
            rotor::Rotor::from_file(rotor_path, "III"),
        ];
        let reflector = rotor::Reflector::from_file(rotor_path, "B");
        let assembly = RotorAssembly::new(rotors, reflector);
        EnigmaAttack::new(Enigma::new(assembly, plugboard), [0, 0, 0])
    }

    fn reset_positions(&mut self) {
        self.enigma.set_positions(self.positions);
    }
}

pub fn known_plaintext_attack(message: &Message, known_plaintext: String) {
    let possible_positions = find_possible_positions(message, &known_plaintext);
    print_possible_positions(message, &known_plaintext, &possible_positions);
    println!("possible positions: {}", possible_positions.len());
    let first_position = possible_positions.get(0).unwrap();
    brute_force_plugboard(message, known_plaintext.as_str(), *first_position);
}

fn brute_force_plugboard(message: &Message, known_plaintext: &str, position: usize) {
    let mut attack = EnigmaAttack::new_default();
    for (crypt_char, clear_char) in
        zip(message.text.chars().skip(position), known_plaintext.chars())
    {
        println!("{} {}", crypt_char, clear_char);
    }
    attack.reset_positions();
}

fn find_possible_positions(message: &Message, known_plaintext: &str) -> Vec<usize> {
    let plaintext_length = known_plaintext.len();
    let max_index = message.text.len() - plaintext_length;
    let mut result = Vec::new();
    for i in 0..max_index {
        if characters_at_each_position_do_not_match(
            &message.text[i..i + plaintext_length],
            known_plaintext,
        ) {
            result.push(i);
        }
    }
    result
}

fn print_possible_positions(message: &Message, known_plaintext: &str, result: &[usize]) {
    for (i, index) in result.iter().enumerate() {
        if i % 10 == 0 {
            println!("{}", message.text);
        }
        for _ in 0..*index {
            print!(" ");
        }
        println!("{}", known_plaintext);
    }
}

fn characters_at_each_position_do_not_match(text_1: &str, text_2: &str) -> bool {
    assert_eq!(text_1.len(), text_2.len());
    assert_ne!(text_1.len(), 0);
    zip(text_1.chars(), text_2.chars()).all(|(x, y)| x != y)
}

#[cfg(test)]
mod tests {
    use crate::cryptoattack::characters_at_each_position_do_not_match;

    #[test]
    fn test_characters_at_each_position_do_not_match() {
        assert!(characters_at_each_position_do_not_match("A", "B"));
        assert!(characters_at_each_position_do_not_match("A", "Z"));
        assert!(characters_at_each_position_do_not_match("ABC", "DEF"));
        assert!(!characters_at_each_position_do_not_match("A", "A"));
        assert!(characters_at_each_position_do_not_match("ABCDE", "ESKXL"));
        assert!(characters_at_each_position_do_not_match("ABCDE", "BCDEA"));
    }

    #[test]
    #[should_panic]
    fn test_characters_at_each_position_do_not_match_unequal_lengths_1() {
        assert!(!characters_at_each_position_do_not_match("ABCDE", "ABCD"));
    }

    #[test]
    #[should_panic]
    fn test_characters_at_each_position_do_not_match_unequal_lengths_2() {
        assert!(!characters_at_each_position_do_not_match("ABCDE", "ABCDEF"));
    }

    #[test]
    #[should_panic]
    fn test_characters_at_each_position_do_not_match_length_zero() {
        assert!(!characters_at_each_position_do_not_match("", ""));
    }
}
