use std::fmt;
use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Pulse {
    Short,      // dots
    Long,       // dash
}

impl Pulse {
    pub fn value(&self) -> &str {
        match self {
            Pulse::Short => ".",
            Pulse::Long => "_",
        }
    }
}
type Letter = Vec<Pulse>;

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}


pub struct MorseCode {
    morse_code: HashMap<char, Letter>,
}

impl MorseCode {
    pub fn new() -> Self {
        let mut morse_code = HashMap::new();
        initialize_morse_code(&mut morse_code);
        Self {
            morse_code
        }
    }

    pub fn get_morse_code(&self, letter:char) -> Option<&Letter> {
        self.morse_code.get(&letter)
    }


    pub fn text_to_morse(&self, text: &str) -> String {
        let mut morse_code = String::new();

        for (i, c) in text.chars().enumerate() {
            if i > 0 {
                morse_code.push(' '); // Add space between characters
            }

            if c.is_whitespace() {
                morse_code.push_str(" "); // Add space between words
                continue;
            }

            if let Some(letter) = self.morse_code.get(&c.to_ascii_uppercase()) {
                morse_code.push_str(
                    &letter
                        .iter()
                        .map(|pulse| pulse.value())
                        .collect::<Vec<_>>()
                        .join(""),
                );
            } else {
                morse_code.push(c); // Keep non-Morse characters as they are
            }
        }

        morse_code
    }

    pub fn display_all_morse_code(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for (key, value) in &self.morse_code {
            writeln!(handle, "Key: {}  {}", key, value.iter().map(|pulse| pulse.value()).collect::<String>()).unwrap_or_else(|e| panic!("Failed to write to stdout: {}", e));
        }
        writeln!(handle).unwrap_or_else(|e| panic!("Failed to write to stdout: {}", e));
    }

    fn is_valid_morse_code(input: &str) -> bool {
        let valid_chars = ['.', '_', ' '];
        for c in input.chars() {
            if !valid_chars.contains(&c) {
                return false;
            }
        }
        true
    }

    pub fn morse_to_text(&self, morse_code: &str) -> Option<String> {
        if !MorseCode::is_valid_morse_code(&morse_code) {
            return None;
        }

        let words: Vec<&str> = morse_code.split("   ").collect();
        let mut decoded_words = String::new();

        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                decoded_words.push(' '); // Add space between words
            }

            let letters: Vec<&str> = word.split(' ').collect();
            for (j, letter) in letters.iter().enumerate() {

                // Find the character (key) corresponding to the Morse code (value)
                let decoded_letter = self
                    .morse_code
                    .iter()
                    .find_map(|(key, value)| {
                        if value.iter().map(|pulse| pulse.value()).collect::<String>() == *letter {
                            Some(key)
                        } else {
                            None
                        }
                    });

                match decoded_letter {
                    Some(character) => decoded_words.push_str(&character.to_string()),
                    None => return None, // Invalid Morse code
                }
            }
        }

        Some(decoded_words)
    }

}

fn initialize_morse_code(morse_code: &mut HashMap<char, Letter>) {
    // Define Morse code patterns for letters and numbers
    let morse_patterns: Vec<(char, Vec<Pulse>)> = vec![
        ('E', vec![Pulse::Short]),
        ('T', vec![Pulse::Long]),
        ('I', vec![Pulse::Short, Pulse::Short]),
        ('A', vec![Pulse::Short, Pulse::Long]),
        ('N', vec![Pulse::Long, Pulse::Short]),
        ('M', vec![Pulse::Long, Pulse::Long]),
        ('S', vec![Pulse::Short, Pulse::Short, Pulse::Short]),
        ('U', vec![Pulse::Short, Pulse::Short, Pulse::Long]),
        ('R', vec![Pulse::Short, Pulse::Long, Pulse::Short]),
        ('W', vec![Pulse::Short, Pulse::Long, Pulse::Long]),
        ('D', vec![Pulse::Long, Pulse::Short, Pulse::Short]),
        ('K', vec![Pulse::Long, Pulse::Short, Pulse::Long]),
        ('G', vec![Pulse::Long, Pulse::Long, Pulse::Short]),
        ('O', vec![Pulse::Long, Pulse::Long, Pulse::Long]),
        ('H', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short]),
        ('V', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long]),
        ('F', vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short]),
        ('L', vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short]),
        ('P', vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short]),
        ('J', vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long]),
        ('B', vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]),
        ('X', vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long]),
        ('C', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]),
        ('Y', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long]),
        ('Z', vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short]),
        ('Q', vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long]),
        ('5', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short]),
        ('4', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long]),
        ('3', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long]),
        ('2', vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long]),
        ('1', vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long]),
        ('6', vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short]),
        ('7', vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]),
        ('8', vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short]),
        ('9', vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Short]),
        ('0', vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long]),
        ('.', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long]),
        (',', vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long]),
        ('?', vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short]),
        ('!', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long]),
        ('&', vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]),
        (':', vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]),
        (';', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]),
        ('(', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short]),
        (')', vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long]),
        ('@', vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]),
        ('$', vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long]),
    ];

    // Iterate through Morse code patterns and insert into the HashMap
    for (character, pulses) in morse_patterns {
        morse_code.insert(character, pulses);
    }
}
