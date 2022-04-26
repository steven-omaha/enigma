mod enigma;
mod plugboard;
mod rotor;
mod rotorassembly;

use crate::enigma::Enigma;

const MESSAGE: &str = "DIESISTEINTEST";

fn main() {
    let mut enigma = Enigma::new();
    let cypher = enigma.encode_message(MESSAGE.to_string());
    println!("{}", cypher);

    let mut enigma = Enigma::new();
    println!("{}", enigma.encode_message(cypher));
}
