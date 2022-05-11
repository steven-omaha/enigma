use std::char;

pub const NUMBER_LETTERS_IN_ALPHABET: usize = 26;

pub const ASCII_LETTER_A: usize = 65;

pub const ALPHABET: [char; NUMBER_LETTERS_IN_ALPHABET] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

///
///
/// # Arguments
///
/// * `input`: any upper case character between A and Z
///
/// returns: usize
///
/// # Examples
///
/// ```
/// assert_eq!(get_position_in_alphabet('A'), 0);
/// assert_eq!(get_position_in_alphabet('B'), 1);
/// assert_eq!(get_position_in_alphabet('Z'), 25);
///
/// ```
pub fn get_position_in_alphabet(input: char) -> usize {
    let upper = input.to_uppercase().next().unwrap() as usize;
    let position_in_alphabet = upper - ASCII_LETTER_A;
    assert!(position_in_alphabet < NUMBER_LETTERS_IN_ALPHABET);
    position_in_alphabet
}

pub fn is_small_letter(input: char) -> bool {
    ('a' .. 'z').contains(&input)
}

pub fn is_capital_letter(input: char) -> bool {
    ('A' .. 'Z').contains(&input)
}

