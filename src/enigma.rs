use crate::alphabet::get_position_in_alphabet;
use crate::plugboard;
use crate::plugboard::Plugboard;
use crate::rotorassembly::RotorAssembly;
use std::path::Path;
use std::str::Chars;

pub struct Enigma {
    assembly: RotorAssembly,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn set_indicator_for_encryption(&mut self, indicator: &str) {
        assert_eq!(indicator.len(), 3);
        let mut positions = [0; 3];
        for (i, char) in indicator.chars().enumerate() {
            positions[i] = get_position_in_alphabet(char);
        }
        self.set_positions(positions);
    }

    pub fn set_indicator_for_decryption(&mut self, indicator: &str) {
        assert_eq!(indicator.len(), 6);
        assert_eq!(
            indicator[0..3],
            indicator[3..6],
            "first half of indicator must equal second half"
        );

        let mut positions = [0; 3];
        for (i, char) in indicator[0..3].chars().enumerate() {
            positions[i] = get_position_in_alphabet(char);
        }
        self.set_positions(positions);
    }

    pub fn get_decrypted_indicator(&mut self, vec: &str) -> String {
        assert_eq!(vec.len(), 6, "initialization vector must be of length 6");
        self.encode_message(vec)
    }

    pub fn get_encrypted_indicator(&mut self, vec: &str) -> String {
        assert_eq!(vec.len(), 3, "initialization vector must be of length 3");
        let mut result1 = self.encode_message(vec);
        let result2 = self.encode_message(vec);
        result1.push_str(result2.as_str());
        result1
    }

    pub fn set_positions(&mut self, positions: [usize; 3]) {
        self.assembly.set_positions(positions);
    }

    pub fn new(assembly: RotorAssembly, plugboard: Plugboard) -> Self {
        Enigma {
            assembly,
            plugboard,
        }
    }

    pub fn new_default() -> Enigma {
        let assembly = RotorAssembly::new_default();
        let plugboard = Plugboard::from_file(Path::new(plugboard::PATH));
        Enigma {
            assembly,
            plugboard,
        }
    }

    pub fn encode_char(&mut self, input: char) -> char {
        self.plugboard
            .encode_char(self.assembly.encode_char(self.plugboard.encode_char(input)))
    }

    pub fn encode_message(&mut self, input: &str) -> String {
        Chars::map(input.chars(), |c| self.encode_char(c)).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Enigma;

    const MESSAGE: &str = "DIESISTEINTESTTESTTEST";

    #[test]
    fn can_encrypt_and_decrypt_char() {
        let input = 'A';

        let mut enigma = Enigma::new_default();
        let cypher = enigma.encode_char(input);

        let mut enigma = Enigma::new_default();
        let output = enigma.encode_char(cypher);

        assert_eq!(input, output);
    }

    #[test]
    fn can_encrypt_and_decrypt_message_with_default_settings() {
        let mut enigma = Enigma::new_default();
        let cypher = enigma.encode_message(MESSAGE);

        let mut enigma = Enigma::new_default();
        let output = enigma.encode_message(cypher.as_str());

        assert_eq!(MESSAGE, output);
    }

    #[test]
    fn can_encrypt_and_decrypt_message_with_random_settings() {
        todo!();

        let mut enigma = Enigma::new_default();
        let cypher = enigma.encode_message(MESSAGE);

        let mut enigma = Enigma::new_default();
        let output = enigma.encode_message(cypher.as_str());

        assert_eq!(MESSAGE.to_string(), output);
    }
}
