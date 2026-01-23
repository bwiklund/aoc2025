struct FileSpan {
    id: Option<i32>,
    len: i32,
}

type Disk = Vec<Option<i32>>;

pub fn solve(part: u32) -> i64 {
    let mut disk = parse_input_sparse().to_disk();

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
                    if first_free_idx >= disk.len() || first_free_idx >= idx {
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

trait ToDisk {
    fn to_disk(&self) -> Disk;
}

impl ToDisk for Vec<FileSpan> {
    fn to_disk(&self) -> Disk {
        self.iter()
            .flat_map(|span| (0..span.len).map(|_| span.id))
            .collect()
    }
}

fn checksum(disk: Disk) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(i, n)| match n {
            Some(n) => i as i64 * *n as i64,
            _ => 0,
        })
        .sum()
}

fn parse_input_sparse() -> Vec<FileSpan> {
    std::fs::read_to_string("./src/day9_input.txt")
        .unwrap()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .map(|(idx, len)| FileSpan {
            id: match idx % 2 == 0 {
                true => Some((idx / 2) as i32),
                false => None,
            },
            len: len as i32,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 6337367222422);
        assert_eq!(solve(1), 0);
    }
}
