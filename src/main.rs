mod plugboard;
mod rotor;
mod rotorassembly;
mod enigma;

use crate::enigma::Enigma;

fn main() {
    let mut enigma = Enigma::new();
    println!("{}", enigma.encode_message("DIESISTEINTEST".to_string()));

    let mut enigma = Enigma::new();
    println!("{}", enigma.encode_message("ODRGYUTQVKOZJR".to_string()));
}
