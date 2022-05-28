use std::fmt;

#[derive(Debug, PartialEq)]
enum Value {
    Spades(u8),
    Clubs(u8),
    Hearts(u8),
    Diamonds(u8),
}

impl Value {
    fn match_enum(suite: &str, numeric_val: u8) -> Value {
        match suite {
            "Spades" => Value::Spades(numeric_val),
            "Clubs" => Value::Clubs(numeric_val),
            "Hearts" => Value::Hearts(numeric_val),
            _ => Value::Diamonds(numeric_val),
        }
    }
}

struct Card {
    value: Value,
    name: String,
}

impl Card {
    fn new(value: Value, name: &str) -> Card {
        Card { value, name: String::from(name) }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Value::Spades(_) => write!(f, "{} of Spades", self.name),
            Value::Clubs(_) => write!(f, "{} of Clubs", self.name),
            Value::Hearts(_) => write!(f, "{} of Hearts", self.name),
            Value::Diamonds(_) => write!(f, "{} of Diamonds", self.name),
        }
    }
}

struct Deck(Vec<Card>);

impl Deck {
    const SUITES: [&'static str; 4] = ["Spades", "Clubs", "Hearts", "Diamonds"];
    const FACE_CARDS: [&'static str; 4] = ["Ace", "King", "Queen", "Jack"];

    fn new() -> Deck {
        let mut deck = Deck(Vec::new());
        for suite in Self::SUITES.iter() {
            for face_card in Self::FACE_CARDS.iter() {
                deck.0.push(Card { 
                    value: Value::match_enum(suite, 10),
                    name: String::from(*face_card),
                });
            }
        }

        deck
    }
}


#[allow(unused)]
pub mod tests {
    use super::*;

    pub fn create_card() {
        let first_card = Card::new(Value::Spades(5), "5");
        assert_eq!(first_card.value, Value::Spades(5));
        assert_eq!(format!("{first_card}"), "5 of Spades");
        let second_card = Card::new(Value::Spades(10), "Ace");
        assert_eq!(format!("{second_card}"), "Ace of Spades");
        assert_ne!(first_card.value, second_card.value);
    }

    pub fn create_deck() {
        let deck = Deck::new();
    }
}
