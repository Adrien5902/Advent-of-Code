use std::sync::Arc;

use aoc::loadinput;

type Pos = (i32, i32);

#[derive(Clone, Debug)]
struct Beam {
    pos: Pos,
    dir: (i32, i32),
}

impl Beam {
    fn move_dir(&self, lines: &Arc<[Arc<[char]>]>) -> Result<Vec<Beam>, &str> {
        let new_x = self.pos.0 + self.dir.0;
        let new_y = self.pos.1 + self.dir.1;

        if new_x < 0 || new_y < 0 || new_x >= lines[0].len() as i32 || new_y >= lines.len() as i32 {
            return Err(&"Beam out or range");
        }

        let mut new_beam = Beam {
            pos: (new_x, new_y),
            dir: self.dir,
        };

        let char = lines[new_beam.pos.1 as usize][new_beam.pos.0 as usize].clone();

        if char == '/' {
            let temp = new_beam.dir.1.clone();
            new_beam.dir.1 = new_beam.dir.0 * -1;
            new_beam.dir.0 = temp * -1;
        } else if char == '\\' {
            let temp = new_beam.dir.1.clone();
            new_beam.dir.1 = new_beam.dir.0;
            new_beam.dir.0 = temp;
        } else if char == '-' && new_beam.dir.1 != 0 {
            let mut split1 = new_beam.clone();
            let mut split2 = new_beam.clone();
            split1.dir = (1, 0);
            split2.dir = (-1, 0);
            return Ok(vec![split1, split2]);
        } else if char == '|' && new_beam.dir.0 != 0 {
            let mut split1 = new_beam.clone();
            let mut split2 = new_beam.clone();
            split1.dir = (0, 1);
            split2.dir = (0, -1);
            return Ok(vec![split1, split2]);
        }

        Ok(vec![new_beam])
    }
}

fn main() {
    let input = loadinput();

    let lines: Arc<[Arc<[char]>]> = input.lines().map(|l| l.chars().collect()).collect();
    let mut energized_tiles: Vec<Beam> = vec![];
    let mut beams: Vec<Beam> = vec![Beam {
        pos: (-1, 0),
        dir: (1, 0),
    }];

    while beams.len() > 0 {
        beams = beams
            .clone()
            .iter()
            .filter_map(|b| {
                match energized_tiles
                    .iter()
                    .find(|ob| ob.pos == b.pos && ob.dir == b.dir)
                {
                    None => {
                        energized_tiles.push(b.clone());
                        Some(b)
                    }
                    Some(_) => None,
                }
            })
            .map(|b| b.to_owned())
            .collect();

        beams = beams
            .iter()
            .filter_map(|beam| match beam.move_dir(&lines) {
                Ok(b) => Some(b),
                Err(_) => None,
            })
            .flatten()
            .collect()
    }

    let mut result: Vec<&Pos> = vec![];
    energized_tiles
        .iter()
        .for_each(|b| match result.iter().find(|pos| pos == &&&b.pos) {
            Some(_) => (),
            None => result.push(&b.pos),
        });

    println!("{:?}", result.len() - 1); // -1 to remove starting tile
}
