#![allow(dead_code)]

#[derive(Debug)]
struct Shape {
    id: u32,
    w: usize,
    h: usize,
    cells: Vec<bool>,
}

#[derive(Debug)]
struct Region {
    w: usize,
    h: usize,
    shape_counts: Vec<u32>,
}

pub fn solve(part: u32) -> u64 {
    let mut shapes = vec![];
    let mut regions = vec![];

    std::fs::read_to_string("./src/day12_input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            if l.is_empty() {
                return;
            } else if l.contains(':') && l.contains('x') {
                let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
                let (w, h) = parts[0].trim_end_matches(':').split_once('x').unwrap();
                regions.push(Region {
                    w: w.parse().unwrap(),
                    h: h.parse().unwrap(),
                    shape_counts: parts[1..].iter().map(|s| s.parse().unwrap()).collect(),
                })
            } else if l.contains(':') && !l.contains('x') {
                shapes.push(Shape {
                    id: l.split_once(':').unwrap().0.parse().unwrap(),
                    w: 0,
                    h: 0,
                    cells: vec![],
                });
            } else if l.contains('.') || l.contains('#') {
                let row: Vec<bool> = l
                    .chars()
                    .map(|ch| match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    })
                    .collect();
                let last = shapes.len() - 1;
                let shape = &mut shapes[last];
                shape.h += 1;
                shape.w = row.len();
                shape.cells.extend(row);
            } else {
                panic!();
            }
        });

    dbg!(shapes);
    dbg!(regions.iter().take(10).collect::<Vec<_>>());

    match part {
        0 => 0,

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
