use aoc::loadinput;
use rayon::prelude::*;

#[derive(Clone, Copy)]
enum Side {
    L,
    R,
}

struct Node {
    n: [u8; 3],
    s: [[u8; 3]; 2],
}

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn llcm(nums: Vec<usize>) -> usize {
    nums.iter().map(|n| *n).reduce(lcm).unwrap_or(nums[0])
}

fn main() {
    let part = 2;
    let input = loadinput("08");
    let lines = input.lines().collect::<Vec<&str>>();

    let sequence: Vec<Side> = lines[0]
        .chars()
        .map(|char| match char {
            'L' => Side::L,
            'R' => Side::R,
            _ => panic!(),
        })
        .collect();

    let nodes: Vec<Node> = lines[2..]
        .iter()
        .map(|n| {
            let [n, s]: [&str; 2] = n.split(" = ").collect::<Vec<&str>>().try_into().unwrap();

            let mut formated_s = s.chars();
            formated_s.next();
            formated_s.next_back();

            Node {
                n: n.chars()
                    .map(|c| ALPHABET.iter().position(|l| *l == c).unwrap() as u8)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap(),
                s: formated_s
                    .as_str()
                    .split(", ")
                    .map(|side| {
                        side.chars()
                            .map(|c| ALPHABET.iter().position(|l| *l == c).unwrap() as u8)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<[u8; 3]>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect();

    let start_nodes = if part == 1 {
        vec![nodes
            .iter()
            .find(|node| node.n.iter().all(|l| *l == 0))
            .unwrap()]
    } else {
        nodes.iter().filter(|node| node.n[2] == 0).collect()
    };

    let checker: fn(&Node) -> bool = if part == 1 {
        |node| node.n.iter().all(|l| *l == 25)
    } else {
        |node| node.n[2] == 25
    };

    let steps = start_nodes
        .par_iter()
        .map(|node| {
            let mut step = 0;
            let mut current_node = *node;

            while !checker(current_node) {
                let new_n = current_node.s[sequence[step % sequence.len()] as usize];

                current_node = nodes.iter().find(|node| node.n == new_n).unwrap();

                step += 1;
            }

            step
        })
        .collect::<Vec<usize>>();

    println!("{:?}", llcm(steps));
}
