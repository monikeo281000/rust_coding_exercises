use std::io::{self, Write};

pub mod MorseCode;

fn main() {
    
    let morse_code = MorseCode::MorseCode::new();


    let cipher_code = morse_code.text_to_morse("Moni keo");

    println!("{}", cipher_code);

    let decode = morse_code.morse_to_text(&cipher_code);

    if let Some(value) = decode {
        println!("{}", value);
    };
}
