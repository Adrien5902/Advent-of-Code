use aoc::loadinput;

fn main() {
    let input = loadinput();
    println!("answer {:?}", day03(&input, 12));
}

fn day03(input: &str, jolt_size: usize) -> i64 {
    let banks = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut jolts = 0;
    for bank in banks {
        let mut last_pos: isize = -1;
        let mut maxes = Vec::new();
        for size in (0..jolt_size).rev() {
            let mut max = 0;
            let mut max_pos = last_pos + 1;
            let range: std::ops::Range<usize> = max_pos as usize..bank.len() - size;

            for (i, char) in bank[range].iter().enumerate() {
                let n = char.to_string().parse().unwrap();
                if n > max {
                    max = n;
                    max_pos = i as isize + last_pos + 1;
                }

                if n == 9 {
                    break;
                }
            }
            maxes.push(max);
            last_pos = max_pos;
        }

        let jolt = maxes
            .into_iter()
            .reduce(|total, max| total * 10 + max)
            .unwrap();
        jolts += jolt;
    }
    jolts
}

#[test]
fn test_with_example() {
    let example = "987654321111111
811111111111119
234234234234278
818181911112111";

    assert!(day03(example, 2) == 357);
    assert!(day03(example, 12) == 3121910778619);
}
