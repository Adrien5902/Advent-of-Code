use aoc::loadinput;
use rayon::{
    self,
    iter::{IntoParallelIterator, ParallelIterator},
};

fn get_location_for_seed(seed: i64, almanac: &Vec<Vec<[i64; 3]>>) -> i64 {
    let mut source = seed;

    for map in almanac {
        let destination = map.iter().find_map(|data| {
            let [dest, source_value, range] = data;
            let diff = source - *source_value;
            if diff >= 0 && diff < *range {
                Some(dest + diff)
            } else {
                None
            }
        });

        source = destination.unwrap_or(source)
    }

    source
}

fn main() {
    let part: u8 = 2;
    let input = loadinput("05");

    let lines = input.lines().collect::<Vec<&str>>();

    let seeds = lines.clone()[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    let maps_lines = lines
        .clone()
        .iter()
        .enumerate()
        .filter_map(|(no, l)| match l.find("map:") {
            Some(_) => Some(no),
            None => None,
        })
        .collect::<Vec<usize>>();

    let almanac = maps_lines
        .clone()
        .iter()
        .enumerate()
        .map(|(i, line_no)| {
            let end_index = if i < maps_lines.len() - 1 {
                maps_lines[i + 1] - 1
            } else {
                lines.len() - 1
            };

            return lines[line_no + 1..end_index]
                .iter()
                .map(|line| {
                    line.split(" ")
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<i64>>()
                        .try_into()
                        .unwrap()
                })
                .collect();
        })
        .collect::<Vec<Vec<[i64; 3]>>>();

    let locations = if part == 1 {
        seeds
            .iter()
            .map(|seed| get_location_for_seed(*seed as i64, &almanac))
            .collect::<Vec<i64>>()
    } else {
        (0..seeds.len() - 1)
            .step_by(2)
            .map(|i| {
                let seed = seeds[i];
                let range = seeds[i + 1];
                println!("processing seed: {} for range {}", seed, range);
                return (0..range)
                    .into_par_iter()
                    .map(|j| get_location_for_seed(seed + j, &almanac))
                    .min()
                    .unwrap();
            })
            .collect::<Vec<i64>>()
    };

    println!("{:?}", locations.iter().min().unwrap());
}
