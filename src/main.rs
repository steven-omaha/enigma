mod enigma;
mod plugboard;
mod rotor;
mod rotorassembly;
mod alphabet;

use crate::enigma::Enigma;
use crate::plugboard::Plugboard;
use std::iter::zip;
use std::path::Path;

const MESSAGE: &str = "MEINETESTNACHRICHTMITWETTERBERICHTDREILITER";
const INITIALIZATION: &str = "QRS";
const ROTOR_SETTINGS: [usize; 3] = [7, 8, 21];

fn main() {
    let mut enigma = build_enigma();

    let encrypted_indicator = enigma.get_encrypted_indicator(INITIALIZATION);
    enigma.set_indicator_for_encryption(INITIALIZATION);

    let cypher = enigma.encode_message(MESSAGE);
    println!("encrypted indicator: {}", encrypted_indicator);
    println!("cypher text: {}", cypher);

    enigma.set_positions(ROTOR_SETTINGS);
    let decrypted_indicator = enigma.get_decrypted_indicator(encrypted_indicator.as_str());
    enigma.set_indicator_for_decryption(decrypted_indicator.as_str());
    let decrypted = enigma.encode_message(cypher.as_str());

    println!("decrypted indicator: {}", decrypted_indicator);
    println!("decrypted message: {}", decrypted);
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
