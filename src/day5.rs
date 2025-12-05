pub fn solve(_part: u32) -> usize {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut is_ids_section = false;

    std::fs::read_to_string("./src/day5_input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line == "" {
                is_ids_section = true
            } else if is_ids_section {
                ids.push(line.parse::<u64>().unwrap())
            } else {
                let (l, r) = line.split_once("-").unwrap();
                ranges.push((l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()));
            }
        });

    // brute force approach
    ids.iter()
        .filter(|id| ranges.iter().any(|(low, high)| *id >= low && *id <= high))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        assert_eq!(solve(0), 698);
    }
}
