use aoc::loadinput;

struct DialTurn {
    direction: TurnDirection,
    amount: i32,
}

impl DialTurn {
    fn get_turn_value(&self) -> i32 {
        match self.direction {
            TurnDirection::Left => -self.amount,
            TurnDirection::Right => self.amount,
        }
    }
}
enum TurnDirection {
    Left,
    Right,
}

impl From<char> for TurnDirection {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

fn main() {
    let part = 2;
    let max = 100;
    let input = loadinput();
    let dial_turns = input.lines().map(|line| {
        let mut chars = line.chars();
        DialTurn {
            direction: chars.next().unwrap().into(),
            amount: chars.collect::<String>().parse().unwrap(),
        }
    });

    let mut counter = 0;
    let mut dial_pos: i32 = 50;
    for turn in dial_turns {
        if part == 2 {
            dial_pos += turn.get_turn_value();
            counter += (dial_pos / 100).abs() + if dial_pos < 0 { 1 } else { 0 };
        }

        dial_pos = (max + dial_pos) % max;

        if part == 1 && dial_pos == 0 {
            counter += 1;
        }
    }

    println!("answer {}", counter);
}
