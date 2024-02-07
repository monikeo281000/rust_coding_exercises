use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum ISBN_type {
    ISBN10,
    ISBN13,
}

#[derive(Debug, PartialEq)]
pub struct ISBN_number {
    number: Option<String>,
    genre: Option<ISBN_type>,
}

impl ISBN_number {
    pub fn new() -> Self {
        Self {
            number: None,
            genre: None,
        }
    }

    pub fn set_number(&mut self, number: String) {
        self.number = Some(number);
    }

    pub fn get_number(&self) -> &Option<String> {
        &self.number
    }

    pub fn set_genre(&mut self, genre: ISBN_type) {
        self.genre = Some(genre);
    }

    pub fn get_genre(&self) -> &str {
        match &self.genre.clone().unwrap() {
            ISBN_type::ISBN10 => "ISBN10",
            ISBN_type::ISBN13 => "ISBN13",
        }
    }

    fn check_digit(&self) -> bool {
        let isbn = &self.number.clone().unwrap().replace("-", "");
        //println!("{}", isbn);
        let mut sum = 0;
        for i in 0..12 {
            let multiplier = if i % 2 == 0 { 1 } else { 3 };
            let digit = isbn
                .chars()
                .nth(i)
                .unwrap() as i32 - '0' as i32;
            sum += multiplier * digit;
            //println!("{} * {}", digit, multiplier);
        }
        //println!("{}", sum);
        let last = isbn.chars().last().unwrap() as i32 - '0' as i32;
        //println!("sum % 10 = {}", sum % 10);
        //println!("last = {}", last);
        10 - (sum % 10) == last
    }

    fn check_len(&self) -> bool {
        let digit_only = &self.number
            .clone()
            .unwrap()
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>();
        //println!("len = {}", digit_only.len());
        digit_only.len() == 13
    }

    pub fn is_valid(&self) -> bool {
        ISBN_number::check_len(&self) && ISBN_number::check_digit(&self)
    }
}

impl Default for ISBN_number {
    fn default() -> Self {
        Self{
            number : Some(String::new()),
            genre : Some(ISBN_type::ISBN13),
        }
    }
}


fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1,3,1,3,1,3,1,3,1,3,1,3];

    let weight_applied: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| x * y)
        .map(|subtotal| subtotal as u32)
        .sum();

    let check_digit = 10 - (weight_applied % 10);

    match check_digit {
        10 => 0_u8,
        x => x as u8,
    }
}
