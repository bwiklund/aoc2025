pub fn solve(part: u32) -> u64 {
    fn max_first(v: &[u64]) -> usize {
        let mut max_idx = 0;
        for idx in 0..v.len() {
            if v[idx] > v[max_idx] {
                max_idx = idx;
            }
        }
        max_idx
    }

    std::fs::read_to_string("./src/day3_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|batteries| {
            // pick the biggest number available, ignoring enough at the end to ensure we have enough digits left
            let n = match part {
                0 => 2,
                1 => 12,
                _ => panic!(),
            };

            let mut digit_idxs = vec![];
            let mut min_idx = 0;
            for i in 0..n {
                let choices = &batteries[min_idx..batteries.len().saturating_sub(n - i - 1)];
                let idx = max_first(choices);
                digit_idxs.push(min_idx + idx);
                min_idx += idx + 1;
            }

            digit_idxs
                .iter()
                .enumerate()
                .map(|(digit, &idx)| batteries[idx] * 10u64.pow((n - digit - 1) as u32))
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        assert_eq!(solve(0), 17343);
        assert_eq!(solve(1), 172664333119298);
    }
}
