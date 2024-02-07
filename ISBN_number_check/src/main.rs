pub mod ISBN;
use ISBN::{ISBN_number, ISBN_type};
use std::io::{self, Write};

fn main() {
    let mut isbn = ISBN_number::new();
    let mut isbn_number = String::new();

    print!("Enter a ISBN number: ");
    io::stdout().flush().expect("ERROR: Failed to flush stdout");
    io::stdin()
        .read_line(&mut isbn_number)
        .expect("ERROR: input");
    let isbn_number = isbn_number.trim().to_string();

    isbn.set_number(isbn_number);
    isbn.set_genre(ISBN_type::ISBN13);

    println!("ISBN = {}", isbn.get_number().clone().unwrap_or("None".to_string()));
    println!("Genre = {}", isbn.get_genre());

    if isbn.is_valid() {
        println!("Valid ISBN");
    } else {
        println!("Invalid ISBN");
    }


}
