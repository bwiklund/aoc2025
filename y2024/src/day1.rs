pub fn solve(part: u32) -> i64 {
    let mut l1 = vec![];
    let mut l2 = vec![];
    std::fs::read_to_string("./src/day1_input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            if let [a, b] = l.split_ascii_whitespace().collect::<Vec<_>>()[..] {
                l1.push(a.parse::<i64>().unwrap());
                l2.push(b.parse::<i64>().unwrap());
            }
        });

    l1.sort();
    l2.sort();

    match part {
        0 => l1.iter().zip(l2).map(|(a, b)| (a - b).abs()).sum(),

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
