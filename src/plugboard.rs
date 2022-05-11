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
        assert_eq!(plugboard.encode_char('A'), 'S');
        assert_eq!(plugboard.encode_char('S'), 'A');
        assert_eq!(plugboard.encode_char('H'), 'L');
        assert_eq!(plugboard.encode_char('Z'), 'Z');
    }
}
