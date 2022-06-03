use crate::deck::{ Card, Deck };
use std::{ fmt, cmp, slice };

#[derive(Clone, PartialEq)]
pub enum Action {
    Hit,
    Stand,
    Surrender,
    DoubleDown,
}

pub enum BetResult {
    Win,
    Lose,
    Surrender,
    StandOff,
    Blackjack,
}

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

    pub fn valid_moves(&self) -> Vec<Action> {
        assert!(self.hand_total() <= 21, "Tried to find moves for a busted player");
        let mut valid_moves = vec![Action::Hit, Action::Stand];
        if self.hand.len() == 2 {
            valid_moves.push(Action::Surrender);
            let double_down_range = 9..=11;
            if self.chips >= self.pot && double_down_range.contains(&self.hand_total()) {
                valid_moves.push(Action::DoubleDown);
            }
        }

        valid_moves
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

    pub fn double_down(&mut self) {
        self.bet(self.pot.unwrap()).unwrap();
    }

    pub fn resolve_bet(&mut self, result: BetResult) -> Result<u32, &str> {
        match self.pot {
            Some(n) if n <= 0 => return Err("Tried to resolve when no bet was made"),
            None => return Err("Tried to resolve bet on a dealer"),
            _ => (),
        }
        let amount = match result {
            BetResult::Win => self.pot.unwrap() * 2,
            BetResult::Lose => 0,
            BetResult::Surrender => self.pot.unwrap() / 2,
            BetResult::Blackjack => (self.pot.unwrap() as f32 * 2.5) as u32,
            BetResult::StandOff => self.pot.unwrap(),
        };
        if let Some(chips) = self.chips.as_mut() {
            *chips += amount;
        }
        if let Some(pot) = self.pot.as_mut() {
            *pot = 0;
        }

        Ok(amount)
    }

    pub fn is_in_pot(&self) -> bool {
        self.pot.unwrap() > 0
    }

    pub fn is_broke(&self) -> bool {
        self.chips.unwrap() == 0
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

    pub fn players_left(&self) -> bool {
        self.0.len() > 0
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

    pub fn check_valid_moves() {
        let mut player = Player::new(500, 0);
        let mut deck = Deck::new();
        deck.deal(2);
        player.get_cards(&mut deck, 2);
        assert!(player.valid_moves().contains(&Action::Hit));
        assert!(player.valid_moves().contains(&Action::Stand));
        assert!(player.valid_moves().contains(&Action::Surrender));
        assert!(player.valid_moves().contains(&Action::DoubleDown));
        player.bet(240);
        assert!(player.valid_moves().contains(&Action::DoubleDown));
        player.bet(80);
        assert!(!player.valid_moves().contains(&Action::DoubleDown));
        player.get_cards(&mut deck, 1);
        assert!(!player.valid_moves().contains(&Action::Surrender));
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
        assert_ne!(player.is_broke(), true);
        player.bet(10);
        assert_eq!(player.chips, Some(0));
        assert_eq!(player.is_broke(), true);
        player.resolve_bet(BetResult::Lose);
        player.chips = Some(50);
        player.bet(10);
        player.double_down();
        assert_eq!(player.pot, Some(20));
    }

    pub fn resolve_bet() {
        let mut player = Player::new(100, 0);
        let no_bet_result = player.resolve_bet(BetResult::StandOff);
        assert_eq!(no_bet_result, Err("Tried to resolve when no bet was made"));
        player.bet(30);
        let normal_bet_result = player.resolve_bet(BetResult::StandOff);
        assert_eq!(normal_bet_result, Ok(30));
        assert_eq!(player.chips, Some(100));
        assert_eq!(player.pot, Some(0));
        player.bet(50);
        player.resolve_bet(BetResult::Win);
        assert_eq!(player.chips, Some(150));
        assert_eq!(player.pot, Some(0));
        player.bet(100);
        player.resolve_bet(BetResult::Surrender);
        assert_eq!(player.chips, Some(100));
        player.bet(10);
        player.resolve_bet(BetResult::Blackjack);
        assert_eq!(player.chips, Some(115));
        let mut dealer = Player::new(0, 0);
        let dealer_resolve_bet_result = dealer.resolve_bet(BetResult::Lose);
        assert_eq!(dealer_resolve_bet_result, Err("Tried to resolve bet on a dealer"));
    }

    pub fn create_player_list() {
        let mut player_list = PlayerList::new(5, 100);
        for (i, player) in player_list.iter_mut().enumerate() {
            assert_eq!(format!("{}", player), format!("Player {}", i+1));
            assert_eq!(player.chips, Some(100));
        }
        assert_eq!(true, player_list.players_left());
    }
}
