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

struct Region {
    cells: HashSet<(i32, i32)>,
    borders: HashSet<(i32, i32, i32, i32)>,
    side_count: i32,
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

    let mut regions = vec![];
    for y in 0..grid.h {
        for x in 0..grid.w {
            let search = *grid.get(x, y).unwrap();
            if search == '.' {
                continue;
            }

            let mut cells = HashSet::new();
            let mut bucket_queue = VecDeque::new();

            bucket_queue.push_back((x, y));
            while let Some((x, y)) = bucket_queue.pop_front() {
                if grid.get(x, y) == Some(&search) {
                    cells.insert((x, y));
                    grid.set(x, y, '.');

                    bucket_queue.push_back((x + 1, y));
                    bucket_queue.push_back((x, y + 1));
                    bucket_queue.push_back((x - 1, y));
                    bucket_queue.push_back((x, y - 1));
                }
            }

            let borders: Vec<_> = cells
                .iter()
                .flat_map(|(x, y)| {
                    let mut cell_borders = vec![];
                    let mut add_border_if_exists = |&x, &y, dx, dy| {
                        if !cells.contains(&(x + dx, y + dy)) {
                            cell_borders.push((x, y, dx, dy))
                        }
                    };
                    add_border_if_exists(x, y, 1, 0);
                    add_border_if_exists(x, y, 0, 1);
                    add_border_if_exists(x, y, -1, 0);
                    add_border_if_exists(x, y, 0, -1);
                    cell_borders
                })
                .collect();

            let mut side_count = 0;
            let mut sides_tmp: HashSet<(i32, i32, i32, i32)> =
                HashSet::from_iter(borders.iter().cloned());
            let mut side_queue: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
            // until we don't have any edges left to count,
            while sides_tmp.len() > 0 {
                // bucket fill
                let first = sides_tmp.iter().next().unwrap();
                side_queue.push_back(*first);
                while let Some((x, y, dx, dy)) = side_queue.pop_front() {
                    sides_tmp.remove(&(x, y, dx, dy));
                    let d1 = (x + dy, y + dx, dx, dy);
                    let d2 = (x - dy, y - dx, dx, dy);
                    if sides_tmp.contains(&d1) {
                        side_queue.push_back(d1);
                    }
                    if sides_tmp.contains(&d2) {
                        side_queue.push_back(d2);
                    }
                }
                side_count += 1;
            }

            if cells.len() > 0 {
                regions.push(Region {
                    cells,
                    borders: HashSet::from_iter(borders.iter().cloned()),
                    side_count,
                });
            }
        }
    }

    match part {
        0 => regions
            .iter()
            .map(|r| r.cells.len() as i64 * r.borders.len() as i64)
            .sum(),

        1 => regions
            .iter()
            .map(|r| r.cells.len() as i64 * r.side_count as i64)
            .sum(),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        assert_eq!(solve(0), 1573474);
        assert_eq!(solve(1), 966476);
    }
}
