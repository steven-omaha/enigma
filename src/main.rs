#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::explicit_iter_loop)]
#![warn(clippy::match_bool)]

mod alphabet;
mod enigma;
mod message;
mod mode;
mod plugboard;
mod rotor;
mod rotorassembly;
mod cryptoattack;

use crate::enigma::Enigma;
use crate::message::{preprocess_for_enigma, Indicator, TEXT};
use crate::plugboard::Plugboard;
use std::iter::zip;
use std::path::Path;
use crate::cryptoattack::known_plaintext_attack;

const INITIALIZATION: &str = "QRS";
const ROTOR_SETTINGS: [usize; 3] = [7, 8, 21];

fn main() {
    let mut enigma = build_enigma();

    let indicator = Indicator::new(INITIALIZATION.to_string());
    let message = message::Message::new(indicator, preprocess_for_enigma(TEXT));
    println!("CLEARTEXT MESSAGE:\n{}", message);
    println!();

    let encrypted_message = enigma.encrypt(message);
    println!("ENCRYPTED MESSAGE:\n{}", encrypted_message);
    println!();

    known_plaintext_attack(encrypted_message, "WETTERBERICHT".to_string());
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

    for (rotor, position) in zip(&mut rotors, ROTOR_SETTINGS) {
        rotor.set_position(position);
    }

    let assembly = rotorassembly::RotorAssembly::new(rotors, reflector);
    Enigma::new(assembly, plugboard)
}
