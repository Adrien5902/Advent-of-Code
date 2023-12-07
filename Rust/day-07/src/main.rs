use aoc::loadinput;

#[derive(Debug)]
struct Deck {
    cards: [u32; 5],
    bid: u32,
}

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
    let card_values: [char; 13] = "AKQJT98765432"
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let input: String = loadinput("07_sample");
    let lines = input.lines().collect::<Vec<&str>>();

    let decks: Vec<Deck> = lines
        .iter()
        .map(|line| {
            let [cards, bid]: [&str; 2] =
                line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
            
            let cards_values = cards
                .chars()
                .map(|c| card_values.iter().position(|char| char == &c).unwrap() as u32)
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();

            let pairs = cards_values.

            let type = match cards_values {
                
            };

            Deck {
                bid: bid.parse().unwrap(),
                cards: cards_values,
            }
        })
        .collect();

    println!("{:?}", decks);
}
