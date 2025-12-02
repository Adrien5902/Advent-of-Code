use aoc::loadinput;

fn get_spaced_nums(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .filter_map(|n| match n.parse() {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .collect()
}

fn get_total_num(input: &str) -> u64 {
    let mut nums = String::from("");
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .for_each(|c| nums.push(c));
    nums.parse().unwrap()
}

fn process_race(time: u64, distance: u64) -> u64 {
    ((0..=time.clone() / 2)
        .into_iter()
        .map(|pressed_time| (time - pressed_time) * pressed_time)
        .filter(|dist| *dist > distance)
        .collect::<Vec<u64>>()
        .len() as u64)
        * 2
        - 1
        + (time.clone() % 2)
}

fn main() {
    let part = 2;
    let input = loadinput();
    let lines = input.lines().collect::<Vec<&str>>();

    let result = if part == 1 {
        let times = get_spaced_nums(lines[0]);
        let distances = get_spaced_nums(lines[1]);

        times
            .iter()
            .enumerate()
            .map(|(no, time)| process_race(*time, distances[no]))
            .reduce(|a, b| a * b)
            .unwrap()
    } else {
        let time = get_total_num(lines[0]);
        let distance = get_total_num(lines[1]);

        process_race(time, distance)
    };

    println!("{:?}", result);
}
