use crate::rotor::{Encode, Reflector, Rotor, PATH};
use std::iter::zip;
use std::path::Path;

pub struct RotorAssembly {
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl RotorAssembly {
    pub fn set_positions(&mut self, positions: [usize; 3]) {
        for (rotor, position) in zip(&mut self.rotors, positions) {
            rotor.set_position(position);
        }
    }

    pub fn new(rotors: Vec<Rotor>, reflector: Reflector) -> Self {
        RotorAssembly { rotors, reflector }
    }

    // used in tests
    #[allow(dead_code)]
    pub fn new_default() -> RotorAssembly {
        let mut rotors = Vec::new();
        let ids = vec!["I", "II", "III"];
        let path = Path::new(PATH);
        for id in ids {
            rotors.push(Rotor::from_file(path, id));
        }
        let reflector = Reflector::from_file(path, "B");
        RotorAssembly { rotors, reflector }
    }

    // false positive with debug assertions
    #[allow(clippy::nonminimal_bool)]
    fn print_dbg_rotor_state(&self) {
        if !(cfg!(debug_assertions)) {
            return;
        }
        for i in 0..3 {
            let rotor = self.rotors.get(i).unwrap();
            println!("{:#?}", rotor);
        }
        println!();
    }

    pub fn encode_char(&mut self, input: char) -> char {
        self.increment_cypher_rotor_positions();
        self.print_dbg_rotor_state();
        let mut output = self.encode_forward(input);
        output = self.reflector.encode_char(output);
        self.encode_reverse(output)
    }

    fn encode_forward(&mut self, input: char) -> char {
        let mut output = input;
        for rotor in &mut self.rotors {
            output = rotor.encode_char(output);
        }
        output
    }

    fn increment_cypher_rotor_positions(&mut self) {
        self.increment_first_rotor();
        self.carry_over_position_increments();
    }

    fn increment_first_rotor(&mut self) {
        self.rotors.get_mut(0).unwrap().increment_position();
    }

    fn carry_over_position_increments(&mut self) {
        for i in 0..self.rotors.len() - 1 {
            let r1 = self.rotors.get_mut(i).unwrap();
            let turnover_has_occurred = r1.turnover_has_occurred();
            r1.reset_turnover_state();

            let r2 = self.rotors.get_mut(i + 1).unwrap();
            if turnover_has_occurred {
                r2.increment_position();
            }
        }
    }

    fn encode_reverse(&mut self, input: char) -> char {
        let mut output = input;
        for rotor in self.rotors.iter_mut().rev() {
            output = rotor.encode_char_reverse(output);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::alphabet::{ALPHABET, NUMBER_LETTERS_IN_ALPHABET};
    use crate::rotorassembly::RotorAssembly;

    #[test]
    fn can_init() {
        RotorAssembly::new_default();
    }

    #[test]
    fn encode_char() {
        let mut assembly = RotorAssembly::new_default();
        let old_position = assembly.rotors.get(0).unwrap().get_position();
        assert_eq!(assembly.encode_char('A'), 'E');
        let new_position = assembly.rotors.get(0).unwrap().get_position();
        assert_eq!(
            (old_position + 1) % NUMBER_LETTERS_IN_ALPHABET,
            new_position
        );
    }

    #[test]
    fn reversible() {
        for char in ALPHABET {
            let input = char;
            let mut assembly = RotorAssembly::new_default();
            let cypher = assembly.encode_char(input);

            let mut assembly = RotorAssembly::new_default();
            let output = assembly.encode_char(cypher);

            assert_eq!(input, output);
            assert_ne!(input, cypher);
        }
    }
}
