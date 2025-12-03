use std::fs::read_to_string;

fn main() {
    println!("AOC2025 Output:");

    println!("{:?}", day1(0));
    println!("{:?}", day1(1));
    println!("{:?}", day2(0));
    println!("{:?}", day2(1));
}

fn day1(part: u32) -> i32 {
    let directions = read_to_string("./src/d1_input.txt")
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

fn day2(part: i32) -> u64 {
    fn is_repetitions(digits: &Vec<u8>, stride: usize) -> bool {
        stride > 0
            && stride < digits.len()
            && digits.len() % stride == 0
            && (0..digits.len()).all(|i| digits[i] == digits[i % stride])
    }

    read_to_string("./src/d2_input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .map(|(min, max)| {
            (min..=max)
                .filter(|id| {
                    let digits = id.to_string().into_bytes();

                    match part {
                        0 => is_repetitions(&digits, digits.len() / 2),
                        1 => (1..=(digits.len() / 2).max(1))
                            .any(|stride| is_repetitions(&digits, stride)),
                        _ => false,
                    }
                })
                .sum::<u64>()
        })
        .sum()
}
