use std::{fs::read};

fn main() {
    let input = String::from_utf8(read("../../Inputs/03.txt").expect("file not found")).expect("");
    let games: Vec<&str> = input.lines().collect();
    let games_subsets = games.iter().enumerate().map(|(id, game)| {
        let gameid = (id as u32) + 1;
        game.to_owned().to_owned()
        .replace(format!("Game {} :", gameid).as_str(), "")
        .split(",")
        .collect::<Vec<&str>>()
    });
}