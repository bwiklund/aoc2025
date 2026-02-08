const SIZE: usize = 70;

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

impl std::fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let ch = match self.get(x, y).unwrap() {
                    true => '#',
                    false => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(part: u32) -> i64 {
    let falling_bytes = parse_input();
    let mut grid = new_empty_grid();

    match part {
        0 => {
            for i in 0..1000 {
                let (x, y) = falling_bytes[i];
                grid.set(x, y, true)
            }
            print!("{:}", grid);
            0
        }

        1 => 0,

        _ => unreachable!(),
    }
}

fn new_empty_grid() -> Grid<bool> {
    Grid {
        cells: (0..SIZE)
            .into_iter()
            .map(|y| (0..SIZE).into_iter().map(|x| false).collect())
            .collect(),
        w: SIZE as i32,
        h: SIZE as i32,
    }
}

fn parse_input() -> Vec<(i32, i32)> {
    std::fs::read_to_string("./src/day18_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(i32, i32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
