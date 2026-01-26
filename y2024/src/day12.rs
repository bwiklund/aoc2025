use std::collections::{HashSet, VecDeque};

struct Grid<T> {
    cells: Vec<Vec<T>>,
    w: i32,
    h: i32,
}

impl<T> Grid<T> {
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            None
        } else {
            Some(&self.cells[y as usize][x as usize])
        }
    }

    fn set(&mut self, x: i32, y: i32, val: T) {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return;
        } else {
            self.cells[y as usize][x as usize] = val;
        }
    }
}

pub fn solve(part: u32) -> i64 {
    let input = std::fs::read_to_string("./src/day12_input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut grid = Grid {
        w: input[0].len() as i32,
        h: input.len() as i32,
        cells: input,
    };

    match part {
        0 => {
            let mut regions = vec![];
            for y in 0..grid.h {
                for x in 0..grid.w {
                    let search = *grid.get(x, y).unwrap();
                    if search == '.' {
                        continue;
                    }

                    let mut region = HashSet::new();
                    let mut bucket_queue = VecDeque::new();

                    bucket_queue.push_back((x, y));
                    while let Some((x, y)) = bucket_queue.pop_front() {
                        if grid.get(x, y) == Some(&search) {
                            region.insert((x, y));
                            grid.set(x, y, '.');

                            bucket_queue.push_back((x + 1, y));
                            bucket_queue.push_back((x - 1, y));
                            bucket_queue.push_back((x, y + 1));
                            bucket_queue.push_back((x, y - 1));
                        }
                    }

                    if region.len() > 0 {
                        regions.push(region);
                    }
                }
            }
            regions
                .iter()
                .map(|r| {
                    let area = r.len();
                    let perimeter = r
                        .iter()
                        .map(|(x, y)| {
                            (!r.contains(&(x + 1, y + 0)) as i64)
                                + (!r.contains(&(x - 1, y + 0)) as i64)
                                + (!r.contains(&(x + 0, y + 1)) as i64)
                                + (!r.contains(&(x + 0, y - 1)) as i64)
                        })
                        .sum::<i64>();
                    area as i64 * perimeter
                })
                .sum()
        }

        1 => 0,

        _ => unreachable!(),
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
