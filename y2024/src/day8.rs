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

    match part {
        0 => {
            let mut unique_antinodes = HashSet::<(i32, i32)>::new();
            for a in nodes.iter() {
                for b in nodes.iter() {
                    if a == b {
                        continue;
                    }
                    if a.2 == b.2 {
                        // calculate one antinode, the one beyond b
                        let anti_x = a.0 + (b.0 - a.0) * 2;
                        let anti_y = a.1 + (b.1 - a.1) * 2;
                        if anti_x >= 0 && anti_x < w && anti_y >= 0 && anti_y < h {
                            unique_antinodes.insert((anti_x, anti_y));
                        }
                    }
                }
            }
            unique_antinodes.len() as i64
        }

        1 => {
            let mut unique_antinodes = HashSet::<(i32, i32)>::new();
            for a in nodes.iter() {
                for b in nodes.iter() {
                    if a == b {
                        continue;
                    }
                    if a.2 == b.2 {
                        // calculate antinodes beyond b until we run off the map
                        let mut x = a.0;
                        let mut y = a.1;
                        let (x2, y2, _) = b;
                        let dx = x2 - x;
                        let dy = y2 - y;

                        loop {
                            x += dx;
                            y += dy;
                            if x >= 0 && x < w && y >= 0 && y < h {
                                unique_antinodes.insert((x, y));
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            unique_antinodes.len() as i64
        }

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        assert_eq!(solve(0), 354);
        assert_eq!(solve(1), 0);
    }
}
