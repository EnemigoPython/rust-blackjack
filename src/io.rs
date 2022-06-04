use crate::player::{ Player, Action };
use std::{ io, str, thread, time };

#[allow(dead_code)]
fn get_user_str(prompt: Option<&str>) -> String {
    match prompt {
        Some(s) => println!("{}", s),
        None => (),
    }

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Problem reading input");

    input
}

#[allow(dead_code)]
fn get_user_int<T>(prompt: Option<&str>) -> T
    where T: str::FromStr
{
    let res = get_user_str(prompt)
        .trim()
        .parse::<T>();
    match res {
        Ok(n) => n,
        Err(_) => get_user_int(prompt),
    }
}

#[allow(dead_code)]
pub fn get_clamped_user_int<T>(prompt: Option<&str>, min: T, max: T) -> T
    where T: str::FromStr + PartialOrd
{
    match get_user_int::<T>(prompt) {
        n if n >= min && n <= max => n,
        _ => get_clamped_user_int(prompt, min, max),
    }
}

#[allow(dead_code)]
pub fn get_user_action(player: &mut Player) -> Action {
    let mut prompt = String::from("Type the number of your desired action:");
    let valid_moves = player.valid_moves();
    for (i, valid_move) in valid_moves.iter().enumerate() {
        prompt.push_str(
            &format!(
                "\n{}: {}", 
                i,
                match valid_move {
                    Action::Hit => "Hit",
                    Action::Stand => "Stand",
                    Action::DoubleDown => "Double Down",
                    Action::Surrender => "Surrender",
                },
            )
        );
    }
    let option = get_clamped_user_int(Some(&prompt), 0, valid_moves.len());

    valid_moves[option].clone()
}

#[allow(dead_code)]
pub fn sleep(duration: u64) {
    let time = time::Duration::from_secs(duration);
    thread::sleep(time);
}
