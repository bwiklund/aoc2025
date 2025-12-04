pub fn solve(part: u32) -> u32 {
    let directions = std::fs::read_to_string("./src/day1_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let sign = match &l[0..1] {
                "R" => 1,
                "L" => -1,
                _ => panic!(),
            };
            let mag: i32 = l[1..].parse().unwrap();
            sign * mag
        })
        .collect::<Vec<i32>>();

    let size = 100;
    let mut pos = 50;
    let mut password = 0;
    for dir in directions {
        let new_pos_unclamped = pos + dir;

        if part == 0 {
            pos = new_pos_unclamped.rem_euclid(size);
            if pos == 0 {
                password += 1;
            }
        }

        if part == 1 {
            let mut dist_remaining_abs = dir.abs();
            while dist_remaining_abs > 0 {
                let max_dist_abs = match pos {
                    0 => size,
                    _ => match dir.signum() {
                        1 => size - pos,
                        -1 => pos,
                        _ => panic!(),
                    },
                };
                let actual_dist_abs = i32::min(max_dist_abs, dist_remaining_abs);
                dist_remaining_abs -= actual_dist_abs;
                pos = (pos + actual_dist_abs * dir.signum()).rem_euclid(size);

                if pos == 0 {
                    password += 1;
                }
            }
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve(0), 1123);
        assert_eq!(solve(1), 6695);
    }
}
