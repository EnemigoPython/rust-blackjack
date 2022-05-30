use std::io as _io;
use std::str;

fn get_user_str(prompt: Option<&str>) -> String {
    match prompt {
        Some(s) => println!("{}", s),
        None => (),
    }

    let mut input = String::new();

    _io::stdin()
        .read_line(&mut input)
        .expect("Problem reading input");

    input
}

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

pub fn get_clamped_user_int<T>(prompt: Option<&str>, min: T, max: T) -> T
    where T: str::FromStr + PartialOrd
{
    match get_user_int::<T>(prompt) {
        n if n >= min && n <= max => n,
        _ => get_clamped_user_int(prompt, min, max),
    }
}
