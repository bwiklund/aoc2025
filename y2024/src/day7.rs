#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn can_be_solved(eq: &Equation, allow_pipe_op: bool, i: usize, acc: u64) -> bool {
    if acc > eq.result {
        return false; // since we are only add+mult of positive numbers, we can never decrease
    }

    if let Some(next) = eq.operands.get(i) {
        can_be_solved(eq, allow_pipe_op, i + 1, acc * next)
            || can_be_solved(eq, allow_pipe_op, i + 1, acc + next)
            || (allow_pipe_op && can_be_solved(eq, allow_pipe_op, i + 1, pipe_op(acc, *next)))
    } else {
        eq.result == acc
    }
}

fn pipe_op(a: u64, b: u64) -> u64 {
    // 123 || 456 = 123456
    let a_mult = 10u64.pow(((b as f64).log10() + 1f64).floor() as u32);
    a * a_mult + b
}

pub fn solve(part: u32) -> u64 {
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
            .filter(|eq| can_be_solved(eq, false, 1, eq.operands[0]))
            .map(|eq| eq.result)
            .sum(),

        1 => eqs
            .iter()
            .filter(|eq| can_be_solved(eq, true, 1, eq.operands[0]))
            .map(|eq| eq.result)
            .sum(),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve(0), 2941973819040);
        assert_eq!(solve(1), 249943041417600);
    }
}
