use std::path::Path;
use crate::plugboard;
use crate::plugboard::Plugboard;
use crate::rotorassembly::RotorAssembly;

pub struct Enigma {
    assembly: RotorAssembly,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new() -> Enigma {
        let assembly = RotorAssembly::new_default();
        let plugboard = Plugboard::from_file(Path::new(plugboard::PATH));
        Enigma { assembly, plugboard }
    }

    pub fn encode_char(&mut self, input: char) -> char {
        self.plugboard.encode_char(self.assembly.encode_char(input))
    }

    pub fn encode_message(&mut self, input: String) -> String {
        input.chars().map(|c| self.encode_char(c)).collect::<String>()
    }
}