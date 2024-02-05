
#[derive(Copy, Clone)]
pub enum CardValue {
    Ace = 1,    // Represents both 1 and 11
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub enum CardType {
    Club,
    Diamond,
    Heart,
    Spades,
}

pub enum CardColor {
    Red,
    Black,
}

impl CardValue {
    pub fn black_jack_value (&self) -> u8 {
        match &self {
            CardValue::Ace => 11,
            _ if (2..=9).contains(&self.value()) => self.value(),
            _ => 10,    // Ten, Jack, Queen, King
        }
    }

    pub fn value (&self) -> u8 {
        match &self {
            CardValue::Ace => 1,
            CardValue::Two | CardValue::Three | CardValue::Four | CardValue::Five | CardValue::Six | CardValue::Seven | CardValue::Eight | CardValue::Nine => {
                *self as u8
            }
            _ => 10
        }
    }
}

pub struct Card {
    value: CardValue,
    shape: CardType,
    color: CardColor,
}

pub struct Hand {
    cards: Vec<CardValue>,
}

impl Hand {
    pub fn new() -> Self {
        let cards: Vec<CardValue> = Vec::new();
        Self {
            cards: cards,
        }
    }

    pub fn value (&self) -> usize {
        let mut total: usize = 0;
        for card in &self.cards {
            total += card.black_jack_value() as usize;    
        }
        total
    }

    pub fn add_card(&mut self, card: CardValue) {
       self.cards.push(card); 
    }
}
