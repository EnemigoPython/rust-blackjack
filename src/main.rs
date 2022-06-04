mod deck;
mod player;
mod io;

use deck::Deck;
use player::{ Player, PlayerList, Action, BetResult };
use io::{ get_clamped_user_int, get_user_action, sleep };

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
        round += 1;
        println!("\nRound {}", round);
        for player in player_list.iter_mut().filter(|p| !p.is_broke()) {
            println!("\n{} to bet, chips: {}", player, player.chips.unwrap());
            let bet = match player.chips.unwrap() {
                n if n <= min_bet => n,
                _ => get_clamped_user_int(
                    Some(&format!("How much would you like to bet? (minimum bet {})", min_bet)), 
                    min_bet, 
                    player.chips.unwrap(),
                ),
            };
            player.bet(bet).unwrap();
        }
        let mut deck = Deck::new();
        deck.shuffle();
        dealer.get_cards(&mut deck, 2);
        for player in player_list.iter_mut().filter(|p| !p.is_broke()) {
            player.get_cards(&mut deck, 2);
            println!("\n{}'s turn:", player);
            println!("Your cards: {}, {} (total: {})", player.hand[0], player.hand[1], player.hand_total());
            println!("Dealer upcard: {}", dealer.hand[0]);
            loop {
                match get_user_action(player) {
                    Action::Hit => {
                        player.get_cards(&mut deck, 1);
                        println!("You get the {} (total: {})", player.latest_card(), player.hand_total());
                        if player.hand_total() > 21 {
                            println!("You went bust!\n");
                            player.resolve_bet(BetResult::Lose).unwrap();
                            break;
                        }
                    },
                    Action::Stand => break,
                    Action::Surrender => {
                        player.resolve_bet(BetResult::Surrender).unwrap();
                        break;
                    },
                    Action::DoubleDown => { 
                        player.double_down();
                        player.get_cards(&mut deck, 1);
                        println!("You get the {}", player.latest_card());
                    },
                }
                sleep(1);
            }
        }
        println!("\nDealer shows the {} (total: {})", dealer.hand[1], dealer.hand_total());
        while dealer.hand_total() < 17 {
            dealer.get_cards(&mut deck, 1);
            println!("Dealer gets the {} (total: {})", dealer.latest_card(), dealer.hand_total());
            if dealer.hand_total() > 21 {
                println!("Dealer busts!");
            }
            sleep(1);
        }
        for player in player_list.iter_mut().filter(|p| p.is_in_pot()) {
            match dealer.hand_total() {
                _ if !dealer.has_blackjack() && player.has_blackjack() => {
                    println!("\nBlackjack for {}!", player);
                    player.resolve_bet(BetResult::Blackjack).unwrap();
                }
                n if n > 21 || n < player.hand_total() => {
                    println!("\n{} wins!", player);
                    player.resolve_bet(BetResult::Win).unwrap();
                },
                n if n == player.hand_total() => {
                    println!("\nStand-off for {}", player);
                    player.resolve_bet(BetResult::StandOff).unwrap();
                },
                _ => {
                    println!("\n{} loses", player);
                    player.resolve_bet(BetResult::Lose).unwrap();
                },
            }
            sleep(1);
        }
        player_list.clear_cards();
        dealer.hand = Vec::new();
        if !player_list.players_left() { 
            break;
        }
    }
    println!("\nThanks for playing!");
}

fn main() {
    println!("Welcome to blackjack!");
    game_loop(init_game_options())
}
