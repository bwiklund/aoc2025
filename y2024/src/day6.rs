use std::collections::HashSet;

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Start,
    Wall,
}

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<Tile>>,
    guard_x: i64,
    guard_y: i64,
    guard_dir: i64,
    visited: HashSet<(i64, i64)>,
    visited_dirs: HashSet<(i64, i64, i64)>,
    // loop_points: HashSet<(i64, i64)>,
}

#[derive(PartialEq)]
enum AdvanceResult {
    Running,
    LeftBoard,
    Looped,
}

impl Grid {
    fn new(str: &str) -> Self {
        let rows: Vec<Vec<Tile>> = str
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| match ch {
                        '.' => Tile::Empty,
                        '^' => Tile::Start,
                        '#' => Tile::Wall,
                        _ => panic!("bad input"),
                    })
                    .collect()
            })
            .collect();
        let (guard_x, guard_y) = rows
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter().enumerate().filter_map(move |(x, cell)| {
                    if *cell == Tile::Start {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()[0];
        let mut grid = Self {
            rows,
            guard_x,
            guard_y,
            guard_dir: 0,
            visited: HashSet::new(),
            visited_dirs: HashSet::new(),
            // loop_points: HashSet::new(),
        };
        grid.mark_path();
        grid
    }

    fn get(&self, x: i64, y: i64) -> Option<&Tile> {
        self.rows
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut Tile> {
        self.rows
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
    }

    fn advance(&mut self) -> AdvanceResult {
        let (dx, dy) = self.guard_dir_to_vel();
        if let Some(Tile::Wall) = self.get(self.guard_x + dx, self.guard_y + dy) {
            self.guard_dir = (self.guard_dir + 1).rem_euclid(4);
        } else {
            self.guard_x += dx;
            self.guard_y += dy;
        }

        if self
            .visited_dirs
            .contains(&(self.guard_x, self.guard_y, self.guard_dir))
        {
            return AdvanceResult::Looped;
        }

        // return whether we're still in bounds
        if self.get(self.guard_x, self.guard_y).is_some() {
            self.mark_path();
            AdvanceResult::Running
        } else {
            AdvanceResult::LeftBoard
        }
    }

    fn mark_path(&mut self) {
        self.visited.insert((self.guard_x, self.guard_y));
        self.visited_dirs
            .insert((self.guard_x, self.guard_y, self.guard_dir));
    }

    fn add_barrier(&mut self, bx: i64, by: i64) {
        self.get_mut(bx, by).map(|t| *t = Tile::Wall);
    }

    fn guard_dir_to_vel(&self) -> (i64, i64) {
        match self.guard_dir.rem_euclid(4) {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => unreachable!(),
        }
    }
}

pub fn solve(part: u32) -> i64 {
    let mut grid = Grid::new(
        std::fs::read_to_string("./src/day6_input.txt")
            .unwrap()
            .as_str(),
    );

    match part {
        0 => {
            while grid.advance() == AdvanceResult::Running {}
            grid.visited.len() as i64
        }

        1 => {
            let mut count = 0i64;
            let mut original_path = grid.clone();

            while original_path.advance() == AdvanceResult::Running {}

            original_path.visited.iter().for_each(|&(x, y)| {
                if x == grid.guard_x && y == grid.guard_y {
                    return;
                }
                let mut cloned = grid.clone();
                cloned.add_barrier(x, y);
                loop {
                    match cloned.advance() {
                        AdvanceResult::Running => continue,
                        AdvanceResult::LeftBoard => break,
                        AdvanceResult::Looped => {
                            count += 1;
                            break;
                        }
                    }
                }
            });
            count
        }

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve(0), 5564);
        assert_eq!(solve(1), 1976);
    }
}
