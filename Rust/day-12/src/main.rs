use aoc::loadinput;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Spring {
    Unknown,
    Working,
    Broken,
}

impl Spring {
    fn is(&self, t: Spring) -> bool {
        *self as u8 == t as u8
    }
}

fn is_valid(springs: Vec<Spring>, arrangment: &Vec<u32>) -> bool {
    let mut broken_arrangment = vec![];
    let mut stack: u32 = 0;
    springs.iter().for_each(|spring| {
        if spring.is(Spring::Broken) {
            stack += 1;
        } else if spring.is(Spring::Working) && stack > 0 {
            broken_arrangment.push(stack);
            stack = 0;
        }
    });

    if stack > 0 {
        broken_arrangment.push(stack)
    }

    broken_arrangment == *arrangment
}

fn main() {
    let input = loadinput("12");
    let lines: Vec<(Vec<Spring>, Vec<u32>)> = input
        .lines()
        .map(|line| {
            let data: Vec<&str> = line.split(" ").collect();
            let springs: Vec<Spring> = data[0]
                .chars()
                .map(|c| match c {
                    '.' => Spring::Working,
                    '#' => Spring::Broken,
                    '?' => Spring::Unknown,
                    _ => Spring::Unknown,
                })
                .collect();
            let arrangment: Vec<u32> = data[1].split(",").map(|c| c.parse().unwrap()).collect();
            (springs, arrangment)
        })
        .collect();

    let result: Vec<u32> = lines
        .par_iter()
        .map(|(springs, arrangment)| {
            let unknown_springs = springs
                .iter()
                .enumerate()
                .filter(|(_, s)| **s as u8 == Spring::Unknown as u8)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            let unknown_springs_count = unknown_springs.len();

            let possiblities = (2 as usize).pow(unknown_springs_count as u32);
            let mut valid_ones = 0;

            for i in 0..possiblities {
                let binary: Vec<char> = format!("{i:#032b}").chars().rev().collect();
                let mut new_arangment = springs.clone();
                let suite =
                    unknown_springs
                        .iter()
                        .enumerate()
                        .map(|(unknown_index, spring_index)| {
                            (
                                *spring_index,
                                match binary[unknown_index] {
                                    '0' => Spring::Working,
                                    '1' => Spring::Broken,
                                    _ => Spring::Broken,
                                },
                            )
                        });

                suite.for_each(|(spring_index, spring)| new_arangment[spring_index] = spring);

                if is_valid(new_arangment, &arrangment) {
                    valid_ones += 1;
                }
            }

            valid_ones
        })
        .collect();

    println!("{:?}", result.iter().map(|n| *n).sum::<u32>());
}
