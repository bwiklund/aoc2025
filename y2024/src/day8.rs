use std::collections::HashSet;

pub fn solve(part: u32) -> i64 {
    let input: Vec<Vec<_>> = std::fs::read_to_string("./src/day8_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => None,
                    ch => Some(ch),
                })
                .collect()
        })
        .collect();

    let w = input[0].len() as i32;
    let h = input.len() as i32;

    let nodes: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &ch)| ch.and_then(|ch| Some((x as i32, y as i32, ch))))
        })
        .collect();

    let mut unique_antinodes = HashSet::<(i32, i32)>::new();
    for a in nodes.iter() {
        for b in nodes.iter() {
            if a == b {
                continue;
            }
            if a.2 == b.2 {
                // calculate antinodes beyond b until we run off the map
                let mut x = b.0;
                let mut y = b.1;
                let dx = x - a.0;
                let dy = y - a.1;

                if part == 1 {
                    unique_antinodes.insert((x, y));
                }

                loop {
                    x += dx;
                    y += dy;
                    if x >= 0 && x < w && y >= 0 && y < h {
                        unique_antinodes.insert((x, y));
                    } else {
                        break;
                    }

                    if part == 0 {
                        break;
                    }
                }
            }
        }
    }
    unique_antinodes.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        assert_eq!(solve(0), 354);
        assert_eq!(solve(1), 1263);
    }
}
