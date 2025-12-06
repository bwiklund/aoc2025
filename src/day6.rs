enum Op {
    Add,
    Multiply,
}

impl Op {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "*" => Some(Self::Multiply),
            _ => None,
        }
    }
}

pub fn solve(part: u32) -> u64 {
    let txt = std::fs::read_to_string("./src/day6_input.txt").unwrap();
    let lines = txt.lines().collect::<Vec<_>>();
    let columns: Vec<Vec<u64>> = match part {
        0 => {
            let rows = lines[0..lines.len() - 1]
                .iter()
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>();

            // transpose
            (0..rows[0].len())
                .map(|col_idx| {
                    (0..rows.len())
                        .map(|row_idx| rows[row_idx][col_idx])
                        .collect()
                })
                .collect()
        }

        1 => {
            let digits_by_row = lines[0..lines.len() - 1]
                .iter()
                .map(|l| l.chars().map(|ch| ch.to_digit(10)).collect())
                .collect::<Vec<Vec<Option<u32>>>>();

            let columns = (0..digits_by_row[0].len())
                .map(|col_idx| {
                    let digits = digits_by_row
                        .iter()
                        .filter_map(move |row| row[col_idx])
                        .collect::<Vec<_>>();

                    if digits.is_empty() {
                        return None;
                    }

                    Some(
                        digits
                            .iter()
                            .enumerate()
                            .map(|(place, n)| 10u32.pow((digits.len() - place - 1) as u32) * n)
                            .sum::<u32>(),
                    )
                })
                .collect::<Vec<_>>()
                .split(|n| n.is_none())
                .map(|col| col.iter().map(|n| n.unwrap() as u64).collect())
                .collect::<Vec<_>>();

            columns
        }

        _ => panic!(),
    };

    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| Op::parse(s).unwrap());

    ops.zip(columns.iter())
        .map(|(op, operands)| match op {
            Op::Add => operands.iter().sum::<u64>(),
            Op::Multiply => operands.iter().product(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve(0), 6891729672676);
        assert_eq!(solve(1), 9770311947567);
    }
}
