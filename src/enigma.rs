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
    pub fn new() -> Enigma {
        let assembly = RotorAssembly::new_default();
        let plugboard = Plugboard::from_file(Path::new(plugboard::PATH));
        Enigma {
            assembly,
            plugboard,
        }
    }

    pub fn encode_char(&mut self, input: char) -> char {
        self.plugboard.encode_char(self.assembly.encode_char(self.plugboard.encode_char(input)))
    }

    pub fn encode_message(&mut self, input: String) -> String {
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

        let mut enigma = Enigma::new();
        let cypher = enigma.encode_char(input);

        let mut enigma = Enigma::new();
        let output = enigma.encode_char(cypher);

        assert_eq!(input, output);
    }

    #[test]
    fn can_encrypt_and_decrypt_message() {
        let input = MESSAGE.to_string();

        let mut enigma = Enigma::new();
        let cypher = enigma.encode_message(input.clone());

        let mut enigma = Enigma::new();
        let output = enigma.encode_message(cypher);

        assert_eq!(input, output);
    }
}
