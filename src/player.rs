use crate::deck::{ Card, Deck };
use std::{ fmt, cmp, slice };

pub struct Player {
    pub hand: Vec<Card>,
    pub chips: Option<u32>,
    number: u8,
    pot: Option<u32>,
}

impl Player {
    pub fn new(starting_chips: u32, number: u8) -> Player {
        Player { 
            hand: Vec::new(), 
            chips: if starting_chips > 0 { Some(starting_chips) } else { None }, 
            number,
            pot: if starting_chips > 0 { Some(0) } else { None },
        }
    }

    pub fn get_cards(&mut self, deck: &mut Deck, n: usize) {
        self.hand.extend(deck.deal(n));
    }

    fn ace_count(&self) -> u8 {
        self.hand.iter()
            .map(|x| x.numeric_value())
            .filter(|&x| x == 11)
            .count() as u8
    }

    pub fn latest_card(&self) -> &Card {
        &self.hand[self.hand.len()-1]
    }

    pub fn hand_total(&self) -> u8 {
        let base_value: u8 = self.hand.iter()
            .map(|x| x.numeric_value())
            .sum();

        if base_value <= 21 { return base_value }

        let ace_reduction = cmp::min(base_value / 10 + 1, self.ace_count()) * 10;
        base_value - ace_reduction
    }

    pub fn bet(&mut self, amount: u32) -> Result<u32, &str> {
        match self.chips {
            Some(n) if amount > n => return Err("Program tried to bet more chips than it has"),
            None => return Err("Program tried to bet as a dealer"),
            _ => (),
        }
        if let Some(chips) = self.chips.as_mut() {
            *chips -= amount;
        }
        if let Some(pot) = self.pot.as_mut() {
            *pot += amount;
        } 
        Ok(amount)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {}", self.number)
    }
}

pub struct PlayerList(Vec<Player>);

impl PlayerList {
    pub fn new(n: u8, starting_chips: u32) -> PlayerList {
        let mut player_list = PlayerList(Vec::new());
        for i in 1..=n {
            player_list.0.push(Player::new(starting_chips, i));
        }

        player_list
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Player> {
        self.0.iter_mut()
    }
}


#[allow(unused)]
pub mod tests {
    use super::*;

    pub fn create_player() {
        let mut player = Player::new(20, 1);
        assert_eq!(player.chips, Some(20));
        assert_eq!(String::from("Player 1"), format!("{}", player));
        if let Some(chips) = player.chips.as_mut() {
            *chips *= 2;
        } 
        assert_eq!(player.chips, Some(40));
    }

    pub fn deal_player_cards() {
        let mut player = Player::new(20, 0);
        let mut deck = Deck::new();
        player.get_cards(&mut deck, 3);
        assert_eq!(player.hand.len(), 3);
        assert_eq!(format!("{}", player.hand[0]), "2 of Diamonds");
        assert_eq!(player.hand_total(), 9);
        let test_vals: [u8; 5] = [17, 21, 18, 26, 27];
        for n in 0..5 {
            let hard_hand = Card::_test_hand(n);
            player.hand = hard_hand;
            assert_eq!(player.hand_total(), test_vals[n as usize]);
        }
        assert_eq!(player.latest_card(), &Card::_last_card())
    }

    pub fn make_bet() {
        let mut player = Player::new(20, 0);
        let overbet_result = player.bet(30);
        assert_eq!(overbet_result, Err("Program tried to bet more chips than it has"));
        let mut dealer = Player::new(0, 0);
        let dealer_bet_result = dealer.bet(30);
        assert_eq!(dealer_bet_result, Err("Program tried to bet as a dealer"));
        let legal_bet_result = player.bet(10);
        assert_eq!(legal_bet_result, Ok(10));
        assert_eq!(player.pot, Some(10));
        assert_eq!(player.chips, Some(10));
        player.bet(10);
        assert_eq!(player.chips, Some(0));
    }

    pub fn create_player_list() {
        let mut player_list = PlayerList::new(5, 100);
        for (i, player) in player_list.iter_mut().enumerate() {
            assert_eq!(format!("{}", player), format!("Player {}", i+1));
            assert_eq!(player.chips, Some(100));
        }
    }
}
