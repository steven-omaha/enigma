use crate::message::Message;
use std::iter::zip;

pub fn known_plaintext_attack(message: &Message, known_plaintext: String) {
    let possible_positions = find_possible_positions(message, &known_plaintext);
    print_possible_positions(message, &known_plaintext, &possible_positions);
    println!("possible positions: {}", possible_positions.len());
}

fn find_possible_positions(message: &Message, known_plaintext: &str) -> Vec<usize> {
    let plaintext_length = known_plaintext.len();
    let max_index = message.text.len() - plaintext_length;
    let mut result = Vec::new();
    for i in 0..max_index {
        if characters_at_each_position_do_not_match(&message.text[i..i + plaintext_length], known_plaintext) {
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
    for (char_1, char_2) in zip(text_1.chars(), text_2.chars()) {
        if char_1 == char_2 {
            return false;
        }
    }
    true
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
