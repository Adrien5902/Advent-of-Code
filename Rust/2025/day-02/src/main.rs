use std::ops::RangeInclusive;

use aoc::{bench, loadinput};

type IdRange<'a> = RangeInclusive<u128>;

fn main() {
    bench(day02);
}

fn day02() {
    let input = loadinput();
    let ranges: Vec<IdRange> = input
        .split(",")
        .map(|s| {
            let [start, end] = s
                .split("-")
                .map(|id| id.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            start..=end
        })
        .collect();

    let mut answer = 0;

    for range in ranges {
        for n in range {
            if !is_id_valid(n) {
                answer += n;
            }
        }
    }

    println!("answer {}", answer);
}

fn is_id_valid(n: u128) -> bool {
    let digits = n.to_string().chars().collect::<Vec<_>>();
    /*  Part 1
    if digits.len() % 2 == 1 {
        return true;
    }

    let half_len = digits.len() / 2;
    if digits[0..half_len] == digits[half_len..digits.len()] {
        return false;
    } */

    for size in 1..=digits.len() / 2 {
        if size > 1 && digits.len() % size != 0 {
            continue;
        }

        let mut i = 0;
        let pattern = &digits[0..size];
        'inner: while (i + 1) * size <= digits.len() {
            if digits[i * size..(i + 1) * size] != *pattern {
                break 'inner;
            }
            i += 1;
        }

        if i >= digits.len() / size {
            return false;
        }
    }

    return true;
}

#[test]
fn test() {
    assert!(!is_id_valid(11));
    assert!(!is_id_valid(22));
    assert!(!is_id_valid(1010));
    assert!(!is_id_valid(1188511885));
    assert!(is_id_valid(9547));
    assert!(!is_id_valid(1212121212))
}
