mod alphabet;
mod enigma;
mod message;
mod mode;
mod plugboard;
mod rotor;
mod rotorassembly;

use crate::enigma::Enigma;
use crate::message::{preprocess_for_enigma, Indicator, TEXT};
use crate::plugboard::Plugboard;
use std::iter::zip;
use std::path::Path;

const INITIALIZATION: &str = "QRS";
const ROTOR_SETTINGS: [usize; 3] = [7, 8, 21];

fn main() {
    let mut enigma = build_enigma();

    let indicator = Indicator {
        value: INITIALIZATION.to_string(),
    };
    let message = message::Message::new(indicator, preprocess_for_enigma(TEXT));
    println!("CLEARTEXT MESSAGE:\n{}", message);
    println!();

    let encrypted_message = enigma.encrypt(message);
    println!("ENCRYPTED MESSAGE:\n{}", encrypted_message);
    println!();

    enigma.set_positions(ROTOR_SETTINGS);
    let decrypted_message = enigma.decrypt(encrypted_message);
    println!("DECRYPTED MESSAGE:\n{}", decrypted_message);
}

fn build_enigma() -> Enigma {
    let plugboard = Plugboard::from_file(Path::new(plugboard::PATH));

    let rotor_path = Path::new(rotor::PATH);
    let mut rotors = vec![
        rotor::Rotor::from_file(rotor_path, "I"),
        rotor::Rotor::from_file(rotor_path, "II"),
        rotor::Rotor::from_file(rotor_path, "III"),
    ];
    let reflector = rotor::Reflector::from_file(rotor_path, "B");

    for (rotor, position) in zip(rotors.iter_mut(), ROTOR_SETTINGS) {
        rotor.set_position(position);
    }

    let assembly = rotorassembly::RotorAssembly::new(rotors, reflector);
    Enigma::new(assembly, plugboard)
}
