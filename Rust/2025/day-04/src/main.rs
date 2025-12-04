use std::{cell::LazyCell, ops::RangeInclusive};

use aoc::loadinput;

fn main() {
    let a = day04(&loadinput());
    let b = day04_part2(&loadinput());
    println!("answer part 1: {} part 2: {}", a, b);
}

const ADJACENT_RANGE: RangeInclusive<isize> = -1..=1;
const ADJACENT: LazyCell<Vec<Pos>> = LazyCell::new(|| {
    ADJACENT_RANGE
        .clone()
        .map(move |n| {
            ADJACENT_RANGE.clone().filter_map(move |m| {
                if (n, m) == (0, 0) {
                    return None;
                }
                Some((n, m))
            })
        })
        .flatten()
        .collect()
});

fn day04(input: &str) -> i32 {
    let paper_grid: Grid<bool> = input.into();

    let mut lifted = 0;
    for y in 0..paper_grid.table.len() {
        for x in 0..paper_grid.table[y].len() {
            let pos: Pos = (x as isize, y as isize);
            if *paper_grid.get(pos) {
                let adjacent_papers = count_adjacent_papers(&paper_grid, pos);

                if adjacent_papers < 4 {
                    lifted += 1;
                }
            }
        }
    }

    lifted
}

fn day04_part2(input: &str) -> i32 {
    let mut paper_grid: Grid<bool> = input.into();
    let mut new_grid = paper_grid.clone();
    let mut total_lifted = 0;
    let mut lifted = 1;

    while lifted != 0 {
        lifted = 0;
        for y in 0..paper_grid.table.len() {
            for x in 0..paper_grid.table[y].len() {
                let pos: Pos = (x as isize, y as isize);
                if *paper_grid.get(pos) {
                    let adjacent_papers = count_adjacent_papers(&paper_grid, pos);

                    if adjacent_papers < 4 {
                        lifted += 1;
                        new_grid.set(pos, false);
                    }
                }
            }
        }

        paper_grid = new_grid.clone();
        total_lifted += lifted;
    }

    total_lifted
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    table: Vec<Vec<T>>,
    size: usize,
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self {
            size: value.len(),
            table: value,
        }
    }
}

impl From<&str> for Grid<bool> {
    fn from(value: &str) -> Self {
        value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '@' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .into()
    }
}

type Pos = (isize, isize);

impl<T> Grid<T> {
    fn delta_pos(&self, (x, y): Pos, (delta_x, delta_y): Pos) -> Option<Pos> {
        let new_x = x + delta_x;
        let new_y = y + delta_y;

        if new_x < 0 || new_x >= self.size as isize {
            return None;
        }

        if new_y < 0 || new_y >= self.size as isize {
            return None;
        }

        Some((new_x, new_y))
    }

    fn get(&self, (x, y): Pos) -> &T {
        &self.table[y as usize][x as usize]
    }

    fn set(&mut self, (x, y): Pos, value: T) {
        self.table[y as usize][x as usize] = value
    }
}

#[test]
fn test() {
    let example = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let grid: Grid<bool> = example.into();
    assert!(count_adjacent_papers(&grid, (0, 0)) == 2);
    assert!(count_adjacent_papers(&grid, (1, 2)) == 7);
    assert!(day04(example) == 13);
    assert!(day04_part2(example) == 43);
}

fn count_adjacent_papers(grid: &Grid<bool>, pos: Pos) -> u32 {
    let mut adjacent_papers = 0;

    for delta_pos in ADJACENT.iter() {
        if grid
            .delta_pos(pos, *delta_pos)
            .map(|p| *grid.get(p))
            .unwrap_or_default()
        {
            adjacent_papers += 1;
        }
    }

    adjacent_papers
}
