use std::collections::HashSet;

enum Tile {
    None,
    Origin,
    Splitter,
}

pub fn solve(_part: u32) -> u64 {
    let env: Vec<Vec<Tile>> = std::fs::read_to_string("./src/day7_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => Tile::None,
                    '^' => Tile::Splitter,
                    'S' => Tile::Origin,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut beam = HashSet::new();
    let mut split_count = 0;
    for row in env {
        for (idx, t) in row.iter().enumerate() {
            match t {
                Tile::None => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 1499);
        // assert_eq!(solve(1), 0);
    }
}
