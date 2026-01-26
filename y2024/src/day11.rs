pub fn solve(part: u32) -> u64 {
    let mut stones = std::fs::read_to_string("./src/day11_input.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    match part {
        0 => {
            for _ in 0..25 {
                let mut new_stones = vec![];
                for i in 0..stones.len() {
                    let (next, extra) = blink(stones[i]);
                    new_stones.push(next);
                    extra.map(|extra| new_stones.push(extra));
                }
                stones = new_stones;
            }
            stones.len() as u64
        }

        1 => 0,

        _ => unreachable!(),
    }
}

fn blink(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }
    let as_str = stone.to_string();
    if as_str.len() % 2 == 0 {
        return (
            as_str[..as_str.len() / 2].parse().unwrap(),
            Some(as_str[as_str.len() / 2..].parse().unwrap()),
        );
    }
    return (stone.checked_mul(2024).unwrap(), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
