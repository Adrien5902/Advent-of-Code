use aoc::loadinput;

fn expand_galaxies(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_universe = universe.clone();
    universe
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| c == &'.'))
        .for_each(|(no, line)| expanded_universe.insert(no, line.clone()));

    (0..universe[0].len()).into_iter().for_each(|char_i| {
        if universe.iter().all(|line| line[char_i] == '.') {
            expanded_universe = expanded_universe
                .iter()
                .map(|line| {
                    let mut l = line.clone();
                    l.insert(char_i, '.');
                    l
                })
                .collect()
        }
    });

    expanded_universe
}

fn main() {
    let input = loadinput();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let universe = expand_galaxies(lines);
    let galaxies: Vec<(usize, usize)> = universe
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            let galaxies_on_line: Vec<(usize, usize)> = line
                .iter()
                .enumerate()
                .filter_map(|(x, char)| if char == &'#' { Some((x, y)) } else { None })
                .collect();

            if galaxies_on_line.len() > 0 {
                Some(galaxies_on_line)
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let mut galaxy_pairs: Vec<(&(usize, usize), &(usize, usize))> = vec![];
    galaxies.iter().for_each(|galaxy| {
        galaxies.iter().for_each(|galaxy_b| {
            if galaxy_b != galaxy
                && match galaxy_pairs.iter().find(|pair| pair.0 == galaxy_b) {
                    Some(_) => false,
                    None => true,
                }
            {
                galaxy_pairs.push((galaxy, galaxy_b))
            }
        })
    });

    let distances: Vec<usize> = galaxy_pairs
        .iter()
        .map(|(a, b)| {
            let x = a.0.abs_diff(b.0);
            let y = a.1.abs_diff(b.1);

            println!(
                "{:?} {:?} {} {} {}",
                a,
                b,
                x,
                y,
                x + y - if (a.0 as i32 - b.0 as i32) < 0 { 1 } else { 0 }
            );
            x + y - if (a.0 as i32 - b.0 as i32) < 0 { 1 } else { 0 }
        })
        .collect();

    println!("{:?}", distances.iter().map(|n| *n).sum::<usize>());
}
