use std::cmp::Ordering;

use aoc::loadinput;

#[derive(Debug, Clone, Copy)]
struct Deck {
    cards: [i32; 5],
    bid: i32,
    t: Type,
}

#[derive(Debug, Clone, Copy)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn main() {
    let part = 2;

    let card_chars = if part == 1 {
        "AKQJT98765432"
    } else {
        "AKQT98765432J"
    };
    let card_values: [char; 13] = card_chars
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let input: String = loadinput("07");
    let lines = input.lines().collect::<Vec<&str>>();

    let decks: Vec<Deck> = lines
        .iter()
        .map(|line| {
            let [cards, bid]: [&str; 2] =
                line.split(" ").collect::<Vec<&str>>().try_into().unwrap();

            let cards_values: [i32; 5] = cards
                .chars()
                .map(|c| card_values.iter().position(|char| char == &c).unwrap() as i32)
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();

            let joker_count = if part == 1 {
                0
            } else {
                cards_values
                    .iter()
                    .filter(|c| c == &&0)
                    .collect::<Vec<&i32>>()
                    .len()
            };

            let start_at = part - 1;
            let pairs = (start_at..card_values.len())
                .into_iter()
                .map(|i| {
                    cards_values
                        .iter()
                        .filter(|c| c == &&(i as i32))
                        .collect::<Vec<&i32>>()
                        .len()
                })
                .filter(|c| c > &0)
                .collect::<Vec<usize>>();

            let mut used_jokers = joker_count;
            let pairsof2count = pairs
                .clone()
                .iter()
                .filter(|c| {
                    let is_pair = **c + used_jokers == 2;
                    if used_jokers > 0 {
                        used_jokers -= 1;
                    }
                    is_pair
                })
                .collect::<Vec<&usize>>()
                .len();

            let hasatrio = pairs
                .clone()
                .iter()
                .filter(|c| **c + joker_count == 3)
                .collect::<Vec<&usize>>()
                .len()
                > 0;

            let t = {
                if pairs.len() == 1 || pairs.iter().max().unwrap_or(&0) + joker_count >= 5 {
                    Type::FiveOfAKind
                } else if pairs
                    .clone()
                    .iter()
                    .filter(|c| **c + joker_count >= 4)
                    .collect::<Vec<&usize>>()
                    .len()
                    > 0
                {
                    Type::FourOfAKind
                } else if hasatrio
                    && pairs
                        .clone()
                        .iter()
                        .filter(|c| **c >= 2)
                        .collect::<Vec<&usize>>()
                        .len()
                        > 1
                {
                    Type::FullHouse
                } else if hasatrio {
                    Type::ThreeOfAKind
                } else if pairsof2count >= 2 {
                    Type::TwoPairs
                } else if pairsof2count == 1 {
                    Type::OnePair
                } else {
                    Type::HighCard
                }
            };

            println!("{:?} {:?} {:?}", cards_values, pairs, t);

            Deck {
                bid: bid.parse().unwrap(),
                cards: cards_values,
                t,
            }
        })
        .collect();

    let mut ranked_decks = decks;
    ranked_decks.sort_by(|a, b| {
        let mut ordering: i32 = (b.t as i32) - (a.t as i32);

        if ordering == 0 {
            let mut i = 0;
            while ordering == 0 && i < a.cards.len() {
                ordering = a.cards[i] - b.cards[i];
                i += 1;
            }
        }

        if ordering > 0 {
            Ordering::Greater
        } else if ordering < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let res = ranked_decks
        .iter()
        .enumerate()
        .map(|(i, d)| d.bid * (i as i32 + 1))
        .sum::<i32>();

    println!("{:?}", res);
}
