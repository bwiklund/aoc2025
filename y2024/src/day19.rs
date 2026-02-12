use std::collections::HashMap;

pub fn solve(part: u32) -> usize {
    let (base_towels, designs) = parse_input();

    match part {
        0 => designs
            .iter()
            .filter(|d| can_make_design(&base_towels, d))
            .count(),

        1 => designs
            .iter()
            .map(|d| can_make_design_count(&base_towels, d, &mut HashMap::new()))
            .sum(),

        _ => unreachable!(),
    }
}

fn can_make_design(parts: &Vec<String>, design_remaining: &str) -> bool {
    for p in parts {
        if design_remaining == p {
            return true;
        }
        if design_remaining.starts_with(p) && can_make_design(parts, &design_remaining[p.len()..]) {
            return true;
        }
    }
    false
}

// new fn to keep the old one simple and fast i guess.
// the main thing here other than naively counting, is that we need to memoize some stuff so this can complete in a human timespan.
// basically, if we've reached abc with a,b,c, then counted all the stuff after that (since we're depth first) we can memoize that information. ie: with L-3 chars left, we already know the count that is deeper. so if you reach it again with, say, ab,c, you can add that to this recursion's count and return.
fn can_make_design_count(
    parts: &Vec<String>,
    design_remaining: &str,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    let mut count = 0;
    for p in parts {
        if let Some(&cached) = memo.get(&design_remaining.len()) {
            return cached;
        }
        if design_remaining == p {
            count += 1;
        } else if design_remaining.starts_with(p) {
            count += can_make_design_count(parts, &design_remaining[p.len()..], memo);
        }
    }
    memo.insert(design_remaining.len(), count);
    count
}

fn parse_input() -> (Vec<String>, Vec<String>) {
    let txt = std::fs::read_to_string("./src/day19_input.txt").unwrap();
    let mut lines = txt.lines();
    let base_towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.into())
        .collect();
    lines.next().unwrap();
    let designs = lines.map(|s| s.into()).collect();
    (base_towels, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19() {
        assert_eq!(solve(0), 240);
        assert_eq!(solve(1), 848076019766013);
    }
}
