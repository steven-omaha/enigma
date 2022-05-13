use crate::alphabet::{get_position_in_alphabet, is_capital_letter};
use crate::message::Message;
use crate::plugboard::Plugboard;
use crate::rotorassembly::RotorAssembly;

pub struct Enigma {
    assembly: RotorAssembly,
    plugboard: Plugboard,
}

enum Mode {
    Encrypt,
    Decrypt,
}

impl Enigma {
    pub fn decrypt(&mut self, message: Message) -> Message {
        let decrypted_indicator = self.get_decrypted_indicator(&message.indicator);
        self.set_indicator(&decrypted_indicator, Mode::Decrypt);
        let text = self.encode_message(&message.text);
        Message::new(decrypted_indicator, text)
    }

    pub fn encrypt(&mut self, message: Message) -> Message {
        let encrypted_indicator = self.get_encrypted_indicator(&message.indicator);
        self.set_indicator(&message.indicator, Mode::Encrypt);
        let text = self.encode_message(&message.text);
        Message::new(encrypted_indicator, text)
    }

    fn set_indicator(&mut self, indicator: &str, mode: Mode) {
        Self::sanity_check_indicator(indicator, mode);
        let mut positions = [0; 3];
        for (i, char) in indicator[0..3].chars().enumerate() {
            positions[i] = get_position_in_alphabet(char);
        }
        self.set_positions(positions);
    }

    fn sanity_check_indicator(indicator: &str, mode: Mode) {
        match mode {
            Mode::Decrypt => {
                assert_eq!(indicator.len(), 6);
                assert_eq!(
                    indicator[0..3],
                    indicator[3..6],
                    "first half of indicator must equal second half"
                );
            },
            Mode::Encrypt => assert_eq!(indicator.len(), 3)
        }
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

    pub fn encode_char(&mut self, input: char) -> char {
        self.plugboard
            .encode_char(self.assembly.encode_char(self.plugboard.encode_char(input)))
    }

    pub fn encode_message(&mut self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.encode_char(sanity_check(c)))
            .collect::<String>()
    }
}

fn sanity_check(input: char) -> char {
    assert!(
        is_capital_letter(input),
        "message must only contain upper case ascii letters"
    );
    input
}

#[cfg(test)]
mod tests {
    use crate::rotorassembly::RotorAssembly;
    use crate::{enigma, plugboard, Enigma, Plugboard};
    use std::path::Path;

    const MESSAGE: &str = "DIESISTEINTESTTESTTEST";

    fn new_default() -> Enigma {
        let assembly = RotorAssembly::new_default();
        let plugboard = Plugboard::from_file(Path::new(plugboard::PATH));
        Enigma {
            assembly,
            plugboard,
        }
    }

    #[test]
    fn can_encrypt_and_decrypt_char() {
        let input = 'A';

        let mut enigma = new_default();
        let cypher = enigma.encode_char(input);

        let mut enigma = new_default();
        let output = enigma.encode_char(cypher);

        assert_eq!(input, output);
    }

    #[test]
    fn can_encrypt_and_decrypt_message_with_default_settings() {
        let mut enigma = new_default();
        let cypher = enigma.encode_message(MESSAGE);

        let mut enigma = new_default();
        let output = enigma.encode_message(cypher.as_str());

        assert_eq!(MESSAGE, output);
    }

    #[ignore]
    #[test]
    fn can_encrypt_and_decrypt_message_with_random_settings() {
        todo!();

        let mut enigma = new_default();
        let cypher = enigma.encode_message(MESSAGE);

        let mut enigma = new_default();
        let output = enigma.encode_message(cypher.as_str());

        assert_eq!(MESSAGE.to_string(), output);
    }

    #[test]
    fn test_sanity_check() {
        enigma::sanity_check('A');
        enigma::sanity_check('B');
        enigma::sanity_check('Z');
    }

    #[test]
    #[should_panic]
    fn test_sanity_check_invalid() {
        enigma::sanity_check('a');
    }
}
