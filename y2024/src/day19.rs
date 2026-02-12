pub fn solve(part: u32) -> usize {
    let (base_towels, designs) = parse_input();

    match part {
        0 => designs
            .iter()
            .filter(|d| can_make_design(&base_towels, d))
            .count(),

        1 => 0,

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
        // assert_eq!(solve(1), 0);
    }
}
