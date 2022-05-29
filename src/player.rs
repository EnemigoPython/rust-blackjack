use crate::deck::{ Card, Deck };

struct Player {
    hand: Vec<Card>,
    chips: u32,
}

impl Player {
    fn new(starting_chips: u32) -> Player {
        Player { hand: Vec::new(), chips: starting_chips }
    }

    fn get_cards(&mut self, deck: &mut Deck, n: usize) {
        self.hand.extend(deck.deal(n));
    }

    fn has_ace(&self) -> bool {
        let has_ace = self.hand.iter()
            .map(|x| x.numeric_value())
            .find(|&x| x == 11);
        match has_ace {
            Some(_) => true,
            None => false,
        }
    }

    fn hand_total(&self) -> u8 {
        let base_value: u8 = self.hand.iter()
            .map(|x| x.numeric_value())
            .sum();
        if base_value > 21 && self.has_ace() {
            base_value - 10
        } else {
            base_value
        }
    }
}


#[allow(unused)]
pub mod tests {
    use super::*;

    pub fn create_player() {
        let mut player = Player::new(20);
        assert_eq!(player.chips, 20);
        player.chips *= 2;
        assert_eq!(player.chips, 40);
    }

    pub fn deal_player_cards() {
        let mut player = Player::new(20);
        let mut deck = Deck::new();
        player.get_cards(&mut deck, 3);
        assert_eq!(player.hand.len(), 3);
        assert_eq!(format!("{}", player.hand[0]), "2 of Diamonds");
        assert_eq!(player.hand_total(), 9);
        let hard_hand = Card::_test_hand();
        player.hand = hard_hand;
        assert_eq!(player.hand_total(), 17);
    }
}
