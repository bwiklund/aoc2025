pub fn solve(part: u32) -> i64 {
    let mut disk = parse_input();

    match part {
        0 => {
            let mut first_free_idx = 0;
            // scan backwards from end
            for idx in (0..disk.len()).rev() {
                if disk[idx].is_some() {
                    // find the next free slot
                    while first_free_idx < disk.len() && disk[first_free_idx].is_some() {
                        first_free_idx += 1;
                    }
                    if first_free_idx >= disk.len() {
                        break;
                    }
                    if first_free_idx >= idx {
                        break;
                    }
                    disk[first_free_idx] = disk[idx];
                    disk[idx] = None;
                }
            }
            checksum(disk)
        }

        1 => 0,

        _ => unreachable!(),
    }
}

fn checksum(disk: Vec<Option<i32>>) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(i, n)| match n {
            Some(n) => i as i64 * *n as i64,
            _ => 0,
        })
        .sum()
}

fn parse_input() -> Vec<Option<i32>> {
    let mut disk = vec![];
    let mut id = 0;
    let mut phase_is_file = true;
    for ch in std::fs::read_to_string("./src/day9_input.txt")
        .unwrap()
        .chars()
    {
        if let Some(count) = ch.to_digit(10) {
            for _ in 0..count {
                if phase_is_file {
                    disk.push(Some(id));
                } else {
                    disk.push(None);
                }
            }
            if phase_is_file {
                id += 1;
            }
            phase_is_file = !phase_is_file;
        }
    }
    disk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 6337367222422);
        // assert_eq!(solve(1), 0);
    }
}
