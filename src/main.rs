mod deck;
mod player;
mod io;

const MAX_PLAYERS: u8 = 8;
const CHIPS_CLAMP: [u32; 2] = [100, 1000];

fn init_game_options() -> (u8, u32) {
    let number_of_players = io::get_clamped_user_int::<u8>(
        Some(&format!("How many players? (max: {})", MAX_PLAYERS)),
        1, 
        MAX_PLAYERS,
    );
    let starting_chips = io::get_clamped_user_int::<u32>(
        Some(&format!("How many starting chips? (min: {}, max: {})", CHIPS_CLAMP[0], CHIPS_CLAMP[1])), 
        CHIPS_CLAMP[0],
        CHIPS_CLAMP[1],
    );

    (number_of_players, starting_chips)
}

fn game_loop(options: (u8, u32)) {
    let (number_of_players, starting_chips) = options;
    loop {
        println!("{} {}", number_of_players, starting_chips);
        break;
    }
}

fn main() {
    game_loop(init_game_options())
}
