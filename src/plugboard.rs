use crate::alphabet::is_capital_letter;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct Plugboard {
    pairs: Vec<Pair>,
}

pub struct Pair {
    char0: char,
    char1: char,
}

impl Pair {
    fn contains(&self, input: char) -> bool {
        input == self.char0 || input == self.char1
    }

    fn swap(&self, input: char) -> Option<char> {
        match input {
            c if c == self.char0 => Some(self.char1),
            c if c == self.char1 => Some(self.char0),
            _ => None,
        }
    }

    pub fn new(char0: char, char1: char) -> Self {
        assert_ne!(char0, char1, "chars cannot be equal");
        Self::assert_capital_letter(char0);
        Self::assert_capital_letter(char1);
        Pair { char0, char1 }
    }

    fn assert_capital_letter(input: char){
        let msg = "must be a capital letter" ;
        assert!(is_capital_letter(input), "{} {}", input, msg);
        }
}

pub(crate) const PATH: &str = "src/plugboard.txt";

impl Plugboard {
    pub fn from_file(path: &Path) -> Plugboard {
        let mut pairs = Vec::new();
        let contents = fs::read_to_string(path).unwrap();
        for line in contents.lines() {
            pairs.push(Self::get_pair_from_line(line));
        }
        Self::new(pairs)
    }

    fn get_pair_from_line(line: &str) -> Pair {
        let mut chars = line.chars();
        let char0 = chars.next().unwrap();
        let char1 = chars.next().unwrap();
        Pair::new(char0, char1)
    }

    pub fn new(pairs: Vec<Pair>) -> Self {
        Self::sanity_check(&pairs);
        Plugboard { pairs }
    }

    fn sanity_check(items: &[Pair]) {
        assert!(!items.is_empty() && items.len() <= 10);

        let mut hs = HashSet::new();
        for pair in items {
            hs.insert(pair.char0);
            hs.insert(pair.char1);
        }
        assert_eq!(
            hs.len(),
            items.len() * 2,
            "check character not used more than once"
        );
    }

    pub fn encode_char(&self, input: char) -> char {
        for pair in &self.pairs {
            if pair.contains(input) {
                return pair.swap(input).unwrap();
            }
        }
        input
    }
}

#[cfg(test)]
mod tests {
    use crate::plugboard::*;

    #[test]
    fn can_load_file() {
        Plugboard::from_file(Path::new(PATH));
    }

    #[test]
    fn encode_char() {
        let plugboard = Plugboard {
            pairs: vec![Pair {
                char0: 'A',
                char1: 'S',
            }],
        };
        test_char(&plugboard, 'A', 'S');
        test_char(&plugboard, 'S', 'A');
        test_char(&plugboard, 'B', 'B');
        test_char(&plugboard, 'Z', 'Z');
    }

    #[test]
    #[should_panic]
    fn pair_with_identical_chars() {
        Pair::new('A', 'A');
    }

    #[test]
    fn pair() {
        let pair = Pair::new('A', 'B');
        assert!(pair.contains('A'));
        assert!(pair.contains('B'));
        assert!(!pair.contains('C'));
    }

    fn test_char(plugboard: &Plugboard, input: char, expected: char) {
        assert_eq!(plugboard.encode_char(input), expected)
    }
}
