///////////////////////////////////
//Advent of code 2023 Rust Day 01//
///////////////////////////////////

use std::{fs, char::{self, from_digit}, u64};

fn main() {
    let part: u8 = 2;
    
    let numbers_map = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let inputpath = "../../Inputs/01.txt";
    let data = String::from_utf8(fs::read(inputpath).expect("file not found")).expect("");
    
    let lines: Vec<&str> = data.split("\r\n").collect();
    let line_numbers = lines.iter().map(|line| {
        let charvec: Vec<char> = line.chars().collect();
        let mut numbers: Vec<char> = Vec::new();
        for i in 0..line.len(){
            let char = charvec[i];
            if char.is_numeric() {
                numbers.push(char)
            }else if part == 2 {
                for (n, number_str) in numbers_map.iter().enumerate(){
                    let number_letters: Vec<char> = number_str.chars().collect();
                    let mut j = 0;
                    while i + j < charvec.len() && charvec[i+j] == number_letters[j] {
                        if j == number_str.len() - 1 {
                            numbers.push(from_digit((n as u32)+1, 10).unwrap());
                            break;
                        }
                        j += 1;
                    }
                }
            }
        }
        return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap_or(numbers.first().unwrap())).parse::<u64>().unwrap();
    });
    let result = line_numbers.reduce(|a, b| {a + b}).unwrap();
    println!("{}", result);
}