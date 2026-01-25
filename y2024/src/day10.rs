use std::collections::HashSet;

struct Grid {
    cells: Vec<Vec<i32>>,
    w: i32,
    h: i32,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            None
        } else {
            Some(self.cells[y as usize][x as usize])
        }
    }
}

fn traverse<F>(grid: &Grid, on_reach_dest: &mut F, x: i32, y: i32, expect_elevation: i32)
where
    F: FnMut((i32, i32)),
{
    if grid.get(x, y) == Some(expect_elevation) {
        if expect_elevation == 9 {
            on_reach_dest((x, y));
            return;
        }
        let next_elev = expect_elevation + 1;
        traverse(grid, on_reach_dest, x + 1, y, next_elev);
        traverse(grid, on_reach_dest, x - 1, y, next_elev);
        traverse(grid, on_reach_dest, x, y + 1, next_elev);
        traverse(grid, on_reach_dest, x, y - 1, next_elev);
    }
}

pub fn solve(part: u32) -> i32 {
    let input = std::fs::read_to_string("./src/day10_input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let grid = Grid {
        w: input[0].len() as i32,
        h: input.len() as i32,
        cells: input,
    };

    match part {
        0 => {
            let mut score = 0;
            let mut counter = HashSet::with_capacity(1000);
            for y in 0..grid.h {
                for x in 0..grid.w {
                    counter.clear();
                    let cb = &mut |(x, y)| {
                        counter.insert((x, y));
                    };
                    traverse(&grid, cb, x, y, 0);
                    score += counter.len() as i32
                }
            }
            score
        }

        1 => {
            let mut score = 0;
            let cb = &mut |_| {
                score += 1;
            };
            for y in 0..grid.h {
                for x in 0..grid.w {
                    traverse(&grid, cb, x, y, 0);
                }
            }
            score
        }

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve(0), 816);
        assert_eq!(solve(1), 1960);
    }
}
