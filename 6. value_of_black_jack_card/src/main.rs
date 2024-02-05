pub mod Card;

fn main() {

    let mut player1 = Card::Hand::new();

    player1.add_card(Card::CardValue::Ace);

    player1.add_card(Card::CardValue::Nine);
    player1.add_card(Card::CardValue::King);
    println!("{}", player1.value());

}
