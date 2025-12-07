use std::collections::{HashMap, HashSet};

enum Tile {
    Empty,
    Origin,
    Splitter,
}

pub fn solve(part: u32) -> u64 {
    let env: Vec<Vec<Tile>> = std::fs::read_to_string("./src/day7_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    'S' => Tile::Origin,
                    '^' => Tile::Splitter,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    match part {
        0 => {
            let mut beam = HashSet::new();
            let mut split_count = 0;
            for row in env {
                for (idx, t) in row.iter().enumerate() {
                    match t {
                        Tile::Empty => {}
                        Tile::Origin => {
                            beam.insert(idx);
                        }
                        Tile::Splitter => {
                            if beam.contains(&idx) {
                                beam.insert(idx - 1);
                                beam.remove(&idx);
                                beam.insert(idx + 1);
                                split_count += 1;
                            }
                        }
                    }
                }
            }
            split_count
        }

        1 => {
            // variant of part one, but the beams can count how many paths took them there, and add them up
            let mut beam = HashMap::new();
            for row in env {
                for (idx, t) in row.iter().enumerate() {
                    match t {
                        Tile::Empty => {}
                        Tile::Origin => {
                            beam.insert(idx, 1);
                        }
                        Tile::Splitter => {
                            if let Some(&tally) = beam.get(&idx) {
                                *beam.entry(idx - 1).or_insert(0) += tally;
                                beam.remove(&idx);
                                *beam.entry(idx + 1).or_insert(0) += tally;
                            }
                        }
                    }
                }
            }
            beam.values().sum()
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 1499);
        assert_eq!(solve(1), 24743903847942);
    }
}
