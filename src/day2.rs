pub fn solve(part: u32) -> u64 {
    fn is_repetitions(digits: &Vec<u8>, stride: usize) -> bool {
        stride > 0
            && stride < digits.len()
            && digits.len() % stride == 0
            && (0..digits.len()).all(|i| digits[i] == digits[i % stride])
    }

    std::fs::read_to_string("./src/day2_input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .map(|(min, max)| {
            (min..=max)
                .filter(|id| {
                    let digits = id.to_string().into_bytes();

                    match part {
                        0 => is_repetitions(&digits, digits.len() / 2),
                        1 => (1..=(digits.len() / 2).max(1))
                            .any(|stride| is_repetitions(&digits, stride)),
                        _ => false,
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        assert_eq!(solve(0), 15873082855);
        assert_eq!(solve(1), 22617871034);
    }
}
