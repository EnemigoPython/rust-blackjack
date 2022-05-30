use std::io as _io;
use std::str;

pub fn get_user_str(prompt: Option<&str>) -> String {
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

pub fn get_user_int<T>(prompt: Option<&str>) -> Result<T, <T as str::FromStr>::Err>
    where T: str::FromStr
{
    get_user_str(prompt)
        .trim()
        .parse::<T>()
}
