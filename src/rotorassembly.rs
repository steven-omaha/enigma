use crate::rotor::{Rotor, ASCII_LETTER_A, NUMBER_LETTERS_IN_ALPHABET, PATH};
use std::path::Path;

pub struct RotorAssembly {
    rotors: Vec<Rotor>,
    reflector: Rotor,
}

impl RotorAssembly {
    pub fn new_default() -> RotorAssembly {
        let mut rotors = Vec::new();
        let ids = vec!["I", "II", "III"];
        let path = Path::new(PATH);
        for id in ids {
            rotors.push(Rotor::from_file(path, id));
        }
        let reflector = Rotor::from_file(path, "B");
        RotorAssembly { rotors, reflector }
    }

    pub fn encode_char(&mut self, input: char) -> char {
        let mut output = self.encode_forward(input);
        output = self.reflector.encode_char(output);
        self.encode_reverse(output)
    }

    fn encode_forward(&mut self, input: char) -> char {
        let output = self.encode_forward_first_rotor(input);
        self.encode_forward_remaining_rotors(output)
    }

    fn encode_forward_remaining_rotors(&mut self, mut output: char) -> char {
        for i in 0..self.rotors.len() - 1 {
            let r1 = self.rotors.get_mut(i).unwrap();
            let turnover_has_occured = r1.turnover_has_occured();
            r1.reset_turnover_state();

            let r2 = self.rotors.get_mut(i + 1).unwrap();
            if turnover_has_occured {
                r2.increment_position();
            }
            output = r2.encode_char(output);
        }
        output
    }

    fn encode_forward_first_rotor(&mut self, input: char) -> char {
        let rotor = self.rotors.get_mut(0).unwrap();
        rotor.increment_position();
        rotor.encode_char(input)
    }

    fn encode_reverse(&mut self, mut output: char) -> char {
        for i in self.rotors.len()..0 {
            println!("{}", i);
            let rotor = self.rotors.get_mut(i).unwrap();
            output = rotor.encode_char_reverse(output)
        }
        output
    }
}


#[cfg(test)]
mod tests {
    use crate::rotorassembly::*;

    #[test]
    fn can_init() {
        RotorAssembly::new_default();
    }

    #[test]
    fn encode_char() {
        let mut assembly = RotorAssembly::new_default();
        assert_eq!(assembly.encode_char('A'), 'I');
    }

    #[test]
    fn reversible() {
        let input = 'A';
        let mut assembly = RotorAssembly::new_default();
        let cypher = assembly.encode_char(input);

        let mut assembly = RotorAssembly::new_default();
        let output = assembly.encode_char(cypher);

        assert_eq!(input, output);
        assert_ne!(input, cypher);
    }

}
