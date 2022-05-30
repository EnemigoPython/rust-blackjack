mod deck;
mod player;
mod io;

const MAX_PLAYERS: u8 = 8;

fn main() {
    let number_of_players = io::get_clamped_user_int::<u8>(
        Some(
            &format!("How many players? (max: {})", MAX_PLAYERS)
        ),
        0, 
        MAX_PLAYERS,
    );
    println!("{}", number_of_players);
}
