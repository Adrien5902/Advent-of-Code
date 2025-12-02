use aoc::loadinput;

struct Subset {
    color: Color,
    count: u32,
}

type Set = Vec<Subset>;
type Game = Vec<Set>;

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let part: u8 = 2;

    let input = loadinput();
    let games: Vec<&str> = input.lines().collect();

    let games_sets: Vec<Game> = games
        .iter()
        .enumerate()
        .map(|(id, game)| {
            let gameid = (id as u32) + 1;

            let game_without_id = game.replace(format!("Game {}: ", gameid).as_str(), "");
            let sets: Vec<&str> = game_without_id.split("; ").collect();

            let subsets: Game = sets
                .iter()
                .map(|set| {
                    return set
                        .split(", ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|subset| {
                            let subsets_values: Vec<&str> = subset.split(" ").collect();
                            return Subset {
                                color: match subsets_values[1] {
                                    "red" => Color::Red,
                                    "green" => Color::Green,
                                    "blue" => Color::Blue,
                                    &_ => Color::Red,
                                },
                                count: u32::from_str_radix(subsets_values[0], 10)
                                    .expect("can't convert number"),
                            };
                        })
                        .collect::<Set>();
                })
                .collect();
            return subsets;
        })
        .collect();

    if part == 1 {
        let result = games_sets.iter().enumerate().map(|(id, game)| {
            let game_id = (id as u32) + 1;
            let is_possible: bool = game
                .iter()
                .map(|set| {
                    set.iter()
                        .map(|subset| {
                            subset.count
                                <= match subset.color {
                                    Color::Red => 12,
                                    Color::Green => 13,
                                    Color::Blue => 14,
                                }
                        })
                        .collect::<Vec<bool>>()
                        .iter()
                        .map(bool::to_owned)
                        .reduce(|a, b| a && b)
                        .unwrap()
                })
                .collect::<Vec<bool>>()
                .iter()
                .map(bool::to_owned)
                .reduce(|a, b| a && b)
                .unwrap();

            // println!("{:?} {:?}", game_id, is_possible);

            return if is_possible { game_id } else { 0 };
        });

        println!("{:?}", result.reduce(|a, b| a + b).unwrap());
    } else if part == 2 {
        let result = games_sets.iter().map(|game| {
            let subsets = game.iter().flatten();
            subsets
                .clone()
                .filter(|ss| matches!(ss.color, Color::Red))
                .reduce(|a, b| if a.count > b.count { a } else { b })
                .unwrap()
                .count
                * subsets
                    .clone()
                    .filter(|ss| matches!(ss.color, Color::Green))
                    .reduce(|a, b| if a.count > b.count { a } else { b })
                    .unwrap()
                    .count
                * subsets
                    .clone()
                    .filter(|ss| matches!(ss.color, Color::Blue))
                    .reduce(|a, b| if a.count > b.count { a } else { b })
                    .unwrap()
                    .count
        });

        println!("{:?}", result.reduce(|a, b| a + b).unwrap());
    }
}
