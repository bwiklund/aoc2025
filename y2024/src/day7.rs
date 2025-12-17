#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn can_be_solved(eq: &Equation, i: usize, acc: i64) -> bool {
    if acc > eq.result {
        return false; // since we are only add+mult of positive numbers, we can never decrease
    }

    if let Some(next) = eq.operands.get(i) {
        can_be_solved(eq, i + 1, acc * next) || can_be_solved(eq, i + 1, acc + next)
    } else {
        eq.result == acc
    }
}

pub fn solve(part: u32) -> i64 {
    let eqs = std::fs::read_to_string("./src/day7_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();

            Equation {
                result: parts.next().unwrap().trim_end_matches(':').parse().unwrap(),
                operands: parts.map(|p| p.parse().unwrap()).collect(),
            }
        })
        .collect::<Vec<Equation>>();

    match part {
        0 => eqs
            .iter()
            .filter(|eq| can_be_solved(eq, 1, eq.operands[0]))
            .map(|eq| eq.result)
            .sum(),

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 2941973819040);
        // assert_eq!(solve(1), 0);
    }
}
