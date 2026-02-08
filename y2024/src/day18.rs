use std::collections::HashMap;

const SIZE: usize = 71;

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

fn to_id(x: i32, y: i32) -> i32 {
    y * SIZE as i32 + x
}

fn from_id(id: i32) -> (i32, i32) {
    (id % SIZE as i32, id / SIZE as i32)
}

pub fn solve_p1(block_count: usize) -> i64 {
    path_length_to_exit(&parse_input(), block_count).unwrap()
}

fn path_length_to_exit(falling_bytes: &Vec<(i32, i32)>, block_count: usize) -> Option<i64> {
    let mut grid = new_empty_grid();

    for i in 0..block_count {
        let (x, y) = falling_bytes[i];
        grid.set(x, y, true)
    }

    let get_paths = |id: i32| -> Vec<i32> {
        let (x, y) = from_id(id);
        let mut paths = vec![];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if grid.get(x + dx, y + dy).map_or(false, |&b| !b) {
                paths.push(to_id(x + dx, y + dy));
            }
        }
        paths
    };

    pathfind(get_paths, 0, SIZE as i32 * SIZE as i32 - 1).map(|p| p.len() as i64 - 1)
}

pub fn solve_p2() -> String {
    let falling_bytes = parse_input();

    // binary search
    let mut low = 0;
    let mut high = falling_bytes.len() - 1;
    let mut result = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if path_length_to_exit(&falling_bytes, mid).is_some() {
            result = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    let (x, y) = falling_bytes[result];
    format!("{},{}", x, y)
}

// i32 is a unique key so this doesn't need to know what kind of grid or graph or whatever this is working against.
fn pathfind(get_paths: impl Fn(i32) -> Vec<i32>, from: i32, to: i32) -> Option<Vec<i32>> {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut queue: Vec<i32> = vec![];

    scores.insert(from, 0);
    queue.push(from);

    while let Some(next) = queue.pop() {
        if next == to {
            let mut path = vec![to];
            while let Some(&score) = scores.get(path.last().unwrap()) {
                if score == 0 {
                    break;
                }
                let prev = get_paths(*path.last().unwrap())
                    .into_iter()
                    .find(|p| scores.get(p).map_or(false, |s| *s == score - 1))
                    .unwrap();
                path.push(prev);
            }
            path.reverse();
            return Some(path);
        }

        let next_score = scores.get(&next).cloned().unwrap_or(0) + 1;
        for p in get_paths(next) {
            if !scores.contains_key(&p) || next_score < scores[&p] {
                scores.insert(p, next_score);
                queue.push(p);
            }
        }

        queue
            .sort_by_key(|&node| std::cmp::Reverse(scores.get(&node).cloned().unwrap_or(i32::MAX)));
    }

    None
}

fn new_empty_grid() -> Grid<bool> {
    Grid {
        cells: (0..SIZE)
            .into_iter()
            .map(|_y| (0..SIZE).into_iter().map(|_x| false).collect())
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
        assert_eq!(solve_p1(1024), 436);
        assert_eq!(solve_p2(), "61,50");
    }
}
