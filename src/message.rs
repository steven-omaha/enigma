use crate::alphabet::{is_capital_letter, is_small_letter};
use crate::mode::Mode;
use std::fmt::{Display, Formatter};

pub const TEXT: &str =
    "Wetterbericht null sechs null null. Wind null drei null, Staerke vier. Leichter Regen. \
    Bedeckt. Gelegentlicher Nebel. Drei Liter.";

pub struct Message {
    pub indicator: Indicator,
    pub text: String,
}

pub struct Indicator {
    value: String,
}

impl Indicator {
    pub fn sanity_check(&self, mode: &Mode) {
        let length = self.value.len();
        match mode {
            Mode::Decrypt => Self::check_length(length, 6),
            Mode::Encrypt => Self::check_length(length, 3),
        }
    }

    pub fn new(value: String) -> Self {
        Self { value }
    }

    fn check_length(length: usize, required_length: usize) {
        let message = "indicator must be of length";
        assert_eq!(length, required_length, "{} {}", message, required_length);
    }

    pub fn get_first_triplet(&self) -> &str {
        &self.value[0..3]
    }

    pub fn get_second_triplet(&self) -> &str {
        assert_eq!(self.value.len(), 6, "cannot get second triplet if indicator is of length 3");
        &self.value[3..6]
    }
}

impl Message {
    pub fn new(indicator: Indicator, text: String) -> Self {
        Message { indicator, text }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indicator: {}  \ntext: {}",
            self.indicator.value, self.text
        ))
    }
}

pub fn preprocess_for_enigma(message: &str) -> String {
    message
        .chars()
        .filter_map(preprocess_char)
        .collect::<String>()
}

fn preprocess_char(input: char) -> Option<char> {
    if is_small_letter(input) {
        return Some(input.to_uppercase().next().unwrap());
    } else if is_capital_letter(input) {
        return Some(input);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::message::preprocess_for_enigma;

    #[test]
    fn test_preprocess_for_enigma() {
        assert_eq!(preprocess_for_enigma("aBc D*\nyz"), "ABCDYZ");
        assert_eq!(preprocess_for_enigma(""), "");
    }
}
