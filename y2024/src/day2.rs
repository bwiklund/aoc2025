pub fn solve(part: u32) -> u64 {
    let reports = std::fs::read_to_string("./src/day2_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    match part {
        0 => reports.iter().filter(|r| is_safe(&r)).count() as u64,

        1 => reports
            .iter()
            .filter(|r| {
                is_safe(r)
                    || (0..r.len()).any(|remove_idx| {
                        is_safe(
                            &r.iter()
                                .enumerate()
                                .filter(|&(idx, _)| idx != remove_idx)
                                .map(|(_, &val)| val)
                                .collect(),
                        )
                    })
            })
            .count() as u64,

        _ => unreachable!(),
    }
}

fn is_safe(r: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = r.windows(2).map(|w| w[1] - w[0]).collect();
    diffs.iter().all(|d| (1..=3).contains(&d.abs()))
        && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        assert_eq!(solve(0), 334);
        assert_eq!(solve(1), 400);
    }
}
