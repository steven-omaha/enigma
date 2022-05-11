use crate::alphabet::{is_capital_letter, is_small_letter};
use std::fmt::{Display, Formatter};

pub const TEXT: &str =
    "Wetterbericht null sechs null null. Wind null drei null, Staerke vier. Leichter Regen. \
    Bedeckt. Gelegentlicher Nebel. Drei Liter.";

pub struct Message {
    pub indicator: String,
    pub text: String,
}

impl Message {
    pub fn new(indicator: String, text: String) -> Self {
        Message { indicator, text }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indicator: {}  \ntext: {}",
            self.indicator, self.text
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
