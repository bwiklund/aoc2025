pub fn solve(part: u32) -> u32 {
    let grid: Vec<_> = std::fs::read_to_string("./src/day4_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '@' => true,
                    '.' => false,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    fn is_occupied(grid: &Vec<Vec<bool>>, x: i32, y: i32) -> bool {
        let h = grid.len() as i32;
        let w = grid[0].len() as i32; // don't care about empty case
        if y < 0 || x < 0 || y >= h || x >= w {
            return false;
        }

        grid[y as usize][x as usize]
    }

    fn neighbor_count(grid: &Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
        let mut count = 0;
        for iy in y - 1..=y + 1 {
            for ix in x - 1..=x + 1 {
                if ix == x && iy == y {
                    continue;
                }
                if is_occupied(grid, ix, iy) {
                    count += 1;
                }
            }
        }
        count
    }

    let h = grid.len() as i32;
    let w = grid[0].len() as i32; // don't care about empty case
    (0..h)
        .map(|iy| {
            (0..w)
                .filter(|&ix| is_occupied(&grid, ix, iy) && neighbor_count(&grid, ix, iy) < 4)
                .count() as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve(0), 9999999);
    }
}
