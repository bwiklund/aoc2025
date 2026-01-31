use std::{collections::HashMap, i32};

#[derive(Debug)]
struct Grid<T> {
    cells: Vec<Vec<T>>,
    w: i32,
    h: i32,
}

#[derive(Debug)]
enum Cell {
    Wall,
    Path,
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

fn dir_to_vec(dir: i8) -> (i32, i32) {
    match dir {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => panic!("bad dir"),
    }
}

// a self sorting heap or whatever would be better but this runs fine for this problem size.
fn take_lowest_score(queue: &mut Vec<(i32, i32, i8, i32)>) -> Option<(i32, i32, i8, i32)> {
    if let Some((idx, _)) = queue.iter().enumerate().min_by(|a, b| a.1.3.cmp(&b.1.3)) {
        Some(queue.remove(idx))
    } else {
        None
    }
}

#[allow(dead_code)]
fn print_progress(grid: &Grid<Cell>, scores: &HashMap<(i32, i32, i8), i32>) {
    for y in 0..grid.h {
        for x in 0..grid.w {
            // if it has a score, print a +. otherwise print the cell
            if scores.contains_key(&(x, y, 0))
                || scores.contains_key(&(x, y, 1))
                || scores.contains_key(&(x, y, 2))
                || scores.contains_key(&(x, y, 3))
            {
                print!("+");
            } else {
                match grid.get(x, y).unwrap() {
                    Cell::Wall => print!("#"),
                    Cell::Path => print!(" "),
                }
            }
        }
        println!();
    }
}

fn pathfind(
    grid: &mut Grid<Cell>,
    (x, y): (i32, i32),
    dir: i8,
    (end_x, end_y): (i32, i32),
) -> Option<i32> {
    let mut scores = HashMap::new();

    let mut queue = vec![];
    queue.push((x, y, dir, 0));

    while let Some((x, y, dir, score)) = take_lowest_score(&mut queue) {
        match grid.get(x, y) {
            Some(Cell::Path) => {
                scores.entry((x, y, dir)).or_insert(i32::MAX);
                let existing_score = scores.get(&(x, y, dir)).unwrap().clone();
                if score >= existing_score {
                    // dbg!("better path at", x, y, dir, score, existing_score);
                    // don't bother, we've been here already and this is worse
                } else {
                    // print_progress(grid, scores);
                    // std::thread::sleep(std::time::Duration::from_millis(100));

                    // record new low score and path further
                    scores.insert((x, y, dir), score);

                    if (end_x, end_y) == (x, y) {
                        // print_progress(grid, scores);
                        // we're done
                        return Some(score);
                    } else {
                        // forward costs 1
                        let (dx, dy) = dir_to_vec(dir);
                        queue.push((x + dx, y + dy, dir, score + 1));

                        // spin left and right
                        queue.push((x, y, (dir - 1).rem_euclid(4), score + 1000));
                        queue.push((x, y, (dir + 1).rem_euclid(4), score + 1000));
                    }
                }
            }
            _ => continue,
        }
    }

    return None;
}

pub fn solve(part: u32) -> i32 {
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    let map_lines: Vec<Vec<_>> = std::fs::read_to_string("./src/day16_input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Path,
                    'S' => {
                        start = (x as i32, y as i32);
                        Cell::Path
                    }
                    'E' => {
                        end = (x as i32, y as i32);
                        Cell::Path
                    }
                    _ => panic!("bad input"),
                })
                .collect()
        })
        .collect();

    let mut grid = Grid {
        w: map_lines[0].len() as i32,
        h: map_lines.len() as i32,
        cells: map_lines,
    };

    match part {
        0 => pathfind(&mut grid, start, 0, end).unwrap(),

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16() {
        assert_eq!(solve(0), 89460);
        // assert_eq!(solve(1), 0);
    }
}
