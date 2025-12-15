use std::collections::HashMap;

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

        1 => {
            let mut l2counts = HashMap::<i64, i64>::new();
            l2.iter().for_each(|n| {
                *l2counts.entry(*n).or_insert(0) += 1;
            });
            l1.iter().map(|n| n * l2counts.get(n).unwrap_or(&0)).sum()
        }

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve(0), 2742123);
        assert_eq!(solve(1), 21328497);
    }
}
