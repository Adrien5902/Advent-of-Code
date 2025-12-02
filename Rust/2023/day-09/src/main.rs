use aoc::loadinput;

fn main() {
    let part = 2;
    let input = loadinput();
    let sequences = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let extrapolated: Vec<i32> = sequences
        .iter()
        .map(|sequence| {
            let mut stack: Vec<i32> = vec![];
            let mut s = sequence.clone();
            while !s.iter().all(|n| n == &0) {
                stack.push(if part == 1 {
                    *s.last().unwrap()
                } else {
                    *s.first().unwrap()
                });

                let mut iter = s.iter();
                iter.next().unwrap();
                s = iter
                    .enumerate()
                    .map(|(i, n)| {
                        let lastn = s[i];
                        n - lastn
                    })
                    .collect();
            }

            if part == 1 {
                stack.iter().sum::<i32>()
            } else {
                stack.push(0);
                stack.iter().rev().map(|n| *n).reduce(|a, b| b - a).unwrap()
            }
        })
        .collect();

    println!("{:?}", extrapolated.iter().sum::<i32>());
}
