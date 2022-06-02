mod deck;
mod player;
mod io;

use deck::Deck;
use player::{ Player, PlayerList };
use io::get_clamped_user_int;

const MAX_PLAYERS: u8 = 8;
const CHIPS_CLAMP: [u32; 2] = [100, 1000];
const MIN_BET_CLAMP: [u32; 2] = [10, 50];

fn init_game_options() -> (u8, u32, u32) {
    let number_of_players = get_clamped_user_int::<u8>(
        Some(&format!("How many players? (max: {})", MAX_PLAYERS)),
        1, 
        MAX_PLAYERS,
    );
    let starting_chips = get_clamped_user_int::<u32>(
        Some(&format!("How many starting chips? (min: {}, max: {})", CHIPS_CLAMP[0], CHIPS_CLAMP[1])), 
        CHIPS_CLAMP[0],
        CHIPS_CLAMP[1],
    );
    let min_bet = get_clamped_user_int::<u32>(
        Some(&format!("What is the minimum bet? (min: {}, max: {})", MIN_BET_CLAMP[0], MIN_BET_CLAMP[1])), 
        MIN_BET_CLAMP[0],
        MIN_BET_CLAMP[1],
    );

    (number_of_players, starting_chips, min_bet)
}

fn game_loop(options: (u8, u32, u32)) {
    let (number_of_players, starting_chips, min_bet) = options;
    let mut player_list = PlayerList::new(number_of_players, starting_chips);
    let mut dealer = Player::new(0, 0);
    let mut round = 0;
    println!("Good luck!");
    loop {
        let mut deck = Deck::new();
        deck.shuffle();
        dealer.get_cards(&mut deck, 2);
        round += 1;
        println!("\nRound {}", round);
        for player in player_list.iter_mut() {
            println!("\n{}'s turn:", player);
            let bet = get_clamped_user_int(
                Some(&format!("How much would you like to bet? (minimum bet {})", min_bet)), 
                0, 
                player.chips.unwrap(),
            );
            player.get_cards(&mut deck, 2);
            println!("Your cards: {}, {}", player.hand[0], player.hand[1]);
            println!("Dealer upcard: {}", dealer.hand[0]);
            println!("Your chips: {}", player.chips.unwrap());
        }
        break;
    }
}

fn main() {
    println!("Welcome to blackjack!");
    game_loop(init_game_options())
}
