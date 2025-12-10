#[derive(Debug)]
struct Machine {
    lights_desired: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}

fn parse_input() -> Vec<Machine> {
    std::fs::read_to_string("./src/day10_input.txt")
        .unwrap()
        .lines()
        .map(
            |l| match l.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                [lights_desired_str, buttons_str @ .., joltage_str] => Machine {
                    lights_desired: lights_desired_str
                        .trim_matches(|c| c == '[' || c == ']')
                        .chars()
                        .map(|s| match s {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect(),
                    buttons: buttons_str
                        .iter()
                        .map(|s| {
                            s.trim_matches(|c| c == '(' || c == ')')
                                .split(',')
                                .map(|s| s.parse().unwrap())
                                .collect()
                        })
                        .collect(),
                    joltage: joltage_str
                        .trim_matches(|c| c == '{' || c == '}')
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect(),
                },
                _ => panic!(),
            },
        )
        .collect()
}

pub fn solve(part: u32) -> u64 {
    let machines: Vec<Machine> = parse_input();

    match part {
        0 => {
            dbg!(machines);
            0
        }

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
