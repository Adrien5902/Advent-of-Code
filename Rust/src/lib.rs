use std::{env::current_dir, fs::read};

pub fn loadinput() -> String {
    let dir = current_dir().unwrap();
    let mut path = dir.into_iter();
    let day_str = path.next_back().unwrap().to_str().unwrap();
    let day = &day_str[day_str.len() - 2..];
    let year = path.next_back().unwrap().to_str().unwrap();
    String::from_utf8(read(format!("../../../Inputs/{year}/{day}.txt")).unwrap()).unwrap()
}
