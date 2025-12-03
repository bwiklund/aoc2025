use std::fs::read_to_string;

pub fn solve(part: u32) -> u64 {
    fn max_first(v: &[u64]) -> (usize, u64) {
        // fixme scan once not twice
        let n1_max = v.iter().max().cloned().unwrap_or(0);
        let n1_idx = v.iter().position(|n| *n == n1_max).unwrap_or_default();
        (n1_idx, n1_max)
    }

    read_to_string("./src/d3_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|batteries| {
            // pick biggest number except the last one
            // pick the biggest number after that number
            let n = 2;

            let mut digit_idxs = vec![];
            let mut min_idx = 0;
            for i in 0..n {
                let n1_choices = &batteries[min_idx..batteries.len().saturating_sub(n - i - 1)];
                let (n1_idx, _) = max_first(n1_choices);
                digit_idxs.push(n1_idx + min_idx);
                min_idx = n1_idx + 1;
            }

            batteries[digit_idxs[0]] * 10 + batteries[digit_idxs[1]]
        })
        .sum::<u64>()
}
