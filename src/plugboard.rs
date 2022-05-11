use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct Plugboard {
    items: Vec<(char, char)>,
}

pub(crate) const PATH: &str = "src/plugboard.txt";

impl Plugboard {
    pub fn from_file(path: &Path) -> Plugboard {
        let mut items = Vec::new();
        let contents = fs::read_to_string(path).unwrap();
        for line in contents.lines() {
            let mut chars = line.chars();
            let char1 = chars.next().unwrap();
            let char2 = chars.next().unwrap();
            items.push((char1, char2));
        }

        Self::sanity_check(&items);
        Plugboard { items }
    }

    fn sanity_check(items: &[(char, char)]) {
        assert!(!items.is_empty() && items.len() <= 10);

        // check character not used more than once
        let mut hs = HashSet::new();
        for pair in items {
            hs.insert(pair.0);
            hs.insert(pair.1);
        }
        assert_eq!(hs.len(), items.len() * 2);
    }

    pub fn encode_char(self: &Plugboard, input: char) -> char {
        self.convert_char(input).unwrap_or(input)
    }

    fn convert_char(&self, input: char) -> Option<char> {
        for pair in &self.items {
            if input == pair.0 {
                return Some(pair.1);
            }
            if input == pair.1 {
                return Some(pair.0);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::plugboard::*;

    #[test]
    fn can_init() {
        Plugboard::from_file(Path::new(PATH));
    }

    #[test]
    fn encode_char() {
        let plugboard = Plugboard::from_file(Path::new(PATH));
        test_char(&plugboard, 'A', 'S');
        test_char(&plugboard, 'S', 'A');
        test_char(&plugboard, 'Q', 'M');
        test_char(&plugboard, 'M', 'Q');
        test_char(&plugboard, 'K', 'P');
        test_char(&plugboard, 'P', 'K');
        test_char(&plugboard, 'R', 'V');
        test_char(&plugboard, 'V', 'R');
        test_char(&plugboard, 'H', 'L');
        test_char(&plugboard, 'L', 'H');
        test_char(&plugboard, 'C', 'O');
        test_char(&plugboard, 'O', 'C');
        test_char(&plugboard, 'N', 'D');
        test_char(&plugboard, 'D', 'N');

        test_char(&plugboard, 'B', 'B');
        test_char(&plugboard, 'E', 'E');
        test_char(&plugboard, 'F', 'F');
        test_char(&plugboard, 'G', 'G');
        test_char(&plugboard, 'I', 'I');
        test_char(&plugboard, 'J', 'J');
        test_char(&plugboard, 'T', 'T');
        test_char(&plugboard, 'U', 'U');
        test_char(&plugboard, 'W', 'W');
        test_char(&plugboard, 'X', 'X');
        test_char(&plugboard, 'Y', 'Y');
        test_char(&plugboard, 'Z', 'Z');
    }

    fn test_char(plugboard: &Plugboard, input: char, expected: char) {
        assert_eq!(plugboard.encode_char(input), expected)
    }
}
