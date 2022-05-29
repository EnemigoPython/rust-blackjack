use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

#[derive(Debug, PartialEq)]
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
                    value: Value::match_enum(suite, if *face_card == "Ace" { 11 } else { 10 }),
                    name: String::from(*face_card),
                });
            }
            for num in 0..9 {
                let _val = 10 - num;
                deck.0.push(Card {
                    value: Value::match_enum(suite, _val),
                    name: format!("{_val}"),
                });
            }
        }

        deck
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    fn deal(&mut self, n: usize) -> Vec<Card> {
        let mut _vec = Vec::new();
        for _ in 0..n {
            match self.0.pop() {
                Some(i) => _vec.push(i),
                None => panic!("Program tried to deal from an empty Deck"),
            }
        }

        _vec
    }
}


#[allow(unused)]
pub mod tests {
    use super::*;

    pub fn create_card() {
        let first_card = Card::new(Value::Spades(5), "5");
        assert_eq!(first_card.value, Value::Spades(5));
        assert_eq!(first_card.name, "5");
        let second_card = Card::new(Value::Spades(10), "Ace");
        assert_eq!(format!("{second_card}"), "Ace of Spades");
        assert_ne!(first_card.value, second_card.value);
    }

    pub fn create_deck() {
        let deck = Deck::new();
        assert_eq!(deck.0[0].name, "Ace");
        assert_eq!(format!("{}", deck.0[0]), "Ace of Spades");
        assert_eq!(deck.0.len(), 52);
        assert_eq!(deck.0[0].value, Value::Spades(11));
        assert_eq!(deck.0[4].value, Value::Spades(10));
    }

    pub fn shuffle_deck() {
        let mut deck = Deck::new();
        deck.shuffle();
        let iter = deck.0.iter()
            .take(13)
            .fold(true, |acc, curr| {
                match curr.value {
                    Value::Spades(_) => if acc { true } else { false }
                    _ => false
                }
            });
        assert_ne!(iter, true);
    }

    pub fn deal_from_deck() {
        let mut deck = Deck::new();
        let dealt_cards = deck.deal(3);
        assert_eq!(dealt_cards, vec![
            Card { value: Value::Diamonds(2), name: String::from("2") },
            Card { value: Value::Diamonds(3), name: String::from("3") },
            Card { value: Value::Diamonds(4), name: String::from("4") },
        ]);
        assert_eq!(deck.0.len(), 49);
        assert_ne!(dealt_cards[2], deck.0[48]);
    }
}
