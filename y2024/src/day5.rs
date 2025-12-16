pub fn solve(part: u32) -> i64 {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut is_updates = false;
    std::fs::read_to_string("./src/day5_input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            if l.is_empty() {
                is_updates = true;
                return;
            }

            if !is_updates {
                let (a, b) = l.split_once('|').unwrap();
                rules.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            } else {
                updates.push(
                    l.split(',')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        });

    match part {
        0 => {
            // brute force first
            updates
                .iter()
                .map(|update| {
                    if rules
                        .iter()
                        .filter(|r| update.contains(&r.0) && update.contains(&r.1))
                        .all(|r| {
                            update.iter().position(|&x| x == r.0)
                                < update.iter().position(|&x| x == r.1)
                        })
                    {
                        update[update.len() / 2]
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        assert_eq!(solve(0), 5087);
        // assert_eq!(solve(1), 0);
    }
}
