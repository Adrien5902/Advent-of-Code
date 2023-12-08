use aoc::loadinput;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Side {
    L,
    R,
}

struct Node<'a> {
    name: &'a str,
    sides: [&'a str; 2],
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
            let [name, sides]: [&str; 2] =
                n.split(" = ").collect::<Vec<&str>>().try_into().unwrap();

            let mut formated_sides = sides.chars();
            formated_sides.next();
            formated_sides.next_back();

            Node {
                name,
                sides: formated_sides
                    .as_str()
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect();

    let mut current_nodes = if part == 1 {
        vec![nodes.iter().find(|node| node.name == "AAA").unwrap()]
    } else {
        nodes
            .iter()
            .filter(|node| node.name.ends_with("A"))
            .collect()
    };

    let mut step = 0;
    let mut condition = true;

    while condition {
        let side = sequence[step % sequence.len()];

        current_nodes = current_nodes
            .par_iter()
            .map(|node| {
                let new_name = node.sides[side as usize];
                nodes.iter().find(|node| node.name == new_name).unwrap()
            })
            .collect();

        println!(
            "{:?}",
            current_nodes.iter().map(|n| n.name).collect::<Vec<&str>>()
        );

        condition = if part == 1 {
            !current_nodes.iter().all(|n| n.name == "ZZZ")
        } else {
            !current_nodes.iter().all(|n| n.name.ends_with("Z"))
        };

        step += 1;
    }

    println!("{:?}", step);
}
