use aoc::loadinput;

#[derive(Clone, Copy, Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

type Pipe = (Side, Side);
type Pos = (i32, i32);

fn main() {
    let input = loadinput();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let starting_pos: Pos = lines
        .iter()
        .enumerate()
        .find_map(
            |(line_no, line)| match line.iter().enumerate().find(|(_, c)| **c == 'S') {
                Some((i, _)) => Some((i as i32, line_no as i32)),
                None => None,
            },
        )
        .unwrap();

    let pipes = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|char| match char {
                    '|' => (Side::Top, Side::Bottom),
                    '-' => (Side::Left, Side::Right),
                    'L' => (Side::Top, Side::Right),
                    'J' => (Side::Top, Side::Left),
                    '7' => (Side::Left, Side::Bottom),
                    'F' => (Side::Right, Side::Bottom),
                    'S' => (Side::Right, Side::Left),
                    '.' => (Side::None, Side::None),
                    _ => (Side::None, Side::None),
                })
                .collect()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let mut current_pos = starting_pos;
    let mut steps = 0;
    let mut last_side = Side::Left;
    while current_pos != starting_pos || steps == 0 {
        let pipe: &Pipe = &pipes[current_pos.1 as usize][current_pos.0 as usize];
        let other_side = if pipe.0 as usize == last_side as usize {
            pipe.1
        } else {
            pipe.0
        };

        let xmotion: i32 = match other_side {
            Side::Left => -1,
            Side::Right => 1,
            _ => 0,
        };
        let ymotion = match other_side {
            Side::Top => -1,
            Side::Bottom => 1,
            _ => 0,
        };

        println!(
            "{} {} {:?} {:?}",
            current_pos.0, current_pos.1, other_side, pipe
        );

        current_pos = (current_pos.0 + xmotion, current_pos.1 + ymotion);
        last_side = match other_side {
            Side::Bottom => Side::Top,
            Side::Top => Side::Bottom,
            Side::Right => Side::Left,
            Side::Left => Side::Right,
            Side::None => Side::None,
        };
        steps += 1;
    }

    println!("{:?}", steps / 2);
}
