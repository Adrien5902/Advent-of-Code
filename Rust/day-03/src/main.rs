use aoc::loadinput;
struct Number {
    line: usize,
    start_index: usize,
    end_index: usize,
    number: u32,
}

#[derive(Copy, Clone, Debug)]
struct Component {
    number: u32,
    line: usize,
    index: usize,
    char: char,
}

fn get_commponents_from_line(
    lines: Vec<&str>,
    line_no: usize,
    left_index: usize,
    right_index: usize,
    number: u32,
) -> Vec<Component> {
    if line_no < lines.len() {
        lines[line_no].chars().collect::<Vec<char>>()[left_index..right_index]
            .to_vec()
            .iter()
            .enumerate()
            .map(|(i, char)| Component {
                number,
                line: line_no,
                index: left_index + i,
                char: *char,
            })
            .collect()
    } else {
        vec![]
    }
}

fn get_commponents_from_number(lines: Vec<&str>, n: &Number) -> Vec<Component> {
    let dot = '.';
    let line = lines[n.line];
    let chars: Vec<char> = line.chars().collect();

    let left_index = {
        if n.start_index > 0 {
            n.start_index - 1
        } else {
            n.start_index
        }
    };

    let right_index = {
        if n.end_index >= chars.len() {
            n.end_index
        } else {
            n.end_index + 1
        }
    };

    let upperline = {
        if n.line > 0 {
            get_commponents_from_line(lines.clone(), n.line - 1, left_index, right_index, n.number)
        } else {
            vec![]
        }
    };

    vec![
        vec![
            Component {
                number: n.number,
                index: left_index,
                line: n.line,
                char: chars[left_index],
            },
            Component {
                number: n.number,
                index: right_index - 1,
                line: n.line,
                char: chars[right_index - 1],
            },
        ],
        upperline,
        get_commponents_from_line(lines.clone(), n.line + 1, left_index, right_index, n.number),
    ]
    .iter()
    .flatten()
    .filter(|c| !(c.char.is_digit(10) || c.char == dot))
    .map(|c| *c)
    .collect()
}

fn main() {
    let data = loadinput("03");
    let lines = data.lines().collect::<Vec<&str>>();
    let mut numbers: Vec<Number> = vec![];
    lines.iter().enumerate().for_each(|(line_no, line)| {
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();
        while i < line.len() {
            let mut char: char = chars[i];
            let mut nums = String::from("");
            let start_index = i;
            while char.is_digit(10) {
                nums.push(char);
                i += 1;
                if i >= chars.len() {
                    break;
                }
                char = chars[i];
            }
            if nums.len() > 0 {
                let end_index: usize = i;
                let number: u32 = nums.parse().unwrap();
                numbers.push(Number {
                    start_index,
                    end_index,
                    line: line_no,
                    number,
                })
            }
            i += 1;
        }
    });

    let numbers_components = numbers
        .iter()
        .map(|n| get_commponents_from_number(lines.clone(), n));

    let part1_res: u32 = numbers_components
        .clone()
        .map(|components| {
            if components.len() > 0 {
                components[0].number
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", part1_res);

    let gears = numbers_components
        .clone()
        .map(|cs| cs.iter().filter(|c| c.char == '*').map(|c| *c).collect())
        .filter(|v: &Vec<Component>| v.len() > 0)
        .flatten();

    let mut standalone_gears: Vec<Component> = vec![];
    gears.clone().for_each(|a| {
        let current_gears = standalone_gears.clone();
        if match current_gears
            .iter()
            .find(|b| a.index == b.index && a.line == b.line)
        {
            Some(_) => false,
            None => true,
        } {
            standalone_gears.push(a.to_owned())
        }
    });

    let ratios = standalone_gears.iter().map(|gear| {
        let adjacent_numbers: Vec<u32> = gears
            .clone()
            .filter(|g| g.index == gear.index && g.line == gear.line)
            .map(|c| c.number)
            .collect();

        if adjacent_numbers.len() == 2 {
            adjacent_numbers[0] * adjacent_numbers[1]
        } else {
            0
        }
    });

    let part2_res: u32 = ratios.sum();

    println!("{:?}", part2_res);
}
