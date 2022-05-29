use crate::deck::{ Card, Deck };

struct Player {
    hand: Vec<Card>,
    chips: Option<u32>,
}

impl Player {
    fn new(starting_chips: u32) -> Player {
        Player { 
            hand: Vec::new(), 
            chips: if starting_chips > 0 { Some(starting_chips) } else { None }, 
        }
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

    fn bet(&mut self, pot: &mut Pot, amount: u32) -> Result<u32, &str> {
        match self.chips {
            Some(n) if amount > n => return Err("Program tried to bet more chips than it has"),
            None => return Err("Program tried to bet as a dealer"),
            _ => (),
        }
        if let Some(chips) = self.chips.as_mut() {
            *chips -= amount;
        } 
        pot.0 += amount;
        Ok(amount)
    }
}

struct Pot(u32);

impl Pot {
    fn new() -> Pot {
        Pot(0)
    }
}


#[allow(unused)]
pub mod tests {
    use super::*;

    pub fn create_player() {
        let mut player = Player::new(20);
        assert_eq!(player.chips, Some(20));
        if let Some(chips) = player.chips.as_mut() {
            *chips *= 2;
        } 
        assert_eq!(player.chips, Some(40));
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

    pub fn make_bet() {
        let mut player = Player::new(20);
        let mut pot = Pot::new();
        let overbet_result = player.bet(&mut pot, 30);
        assert_eq!(overbet_result, Err("Program tried to bet more chips than it has"));
        let mut dealer = Player::new(0);
        let dealer_bet_result = dealer.bet(&mut pot, 30);
        assert_eq!(dealer_bet_result, Err("Program tried to bet as a dealer"));
        let legal_bet_result = player.bet(&mut pot, 10);
        assert_eq!(legal_bet_result, Ok(10));
    }
}
