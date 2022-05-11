mod enigma;
mod plugboard;
mod rotor;
mod rotorassembly;

use crate::enigma::Enigma;
use crate::plugboard::Plugboard;
use std::iter::zip;
use std::path::Path;

const MESSAGE: &str = "MEINETESTNACHRICHTMITWETTERBERICHTDREILITER";
const ROTOR_SETTINGS: [usize; 3] = [7, 8, 21];

fn main() {
    let mut enigma = build_enigma();

    let message = MESSAGE.to_string();
    let cypher = enigma.encode_message(message);
    println!("{}", cypher);

    enigma.set_positions(ROTOR_SETTINGS);
    let decrypted = enigma.encode_message(cypher);

    println!("{}", decrypted);
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
