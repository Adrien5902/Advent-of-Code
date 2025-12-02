use aoc::loadinput;

fn main() {
    let input = loadinput();
    let patterns: Vec<Vec<&str>> = input
        .split("\r\n\r\n")
        .map(|patern| patern.lines().collect())
        .collect();

    let res = patterns.iter().map(|pattern| {
        let mut in_mirror = false;
        let i = 0;
        while !in_mirror ||  {
            pattern
            i += 1;
        }
    });
}
