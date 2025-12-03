use std::fs::read_to_string;

fn main() {
    println!("AOC2025 Output:");

    println!("{:?}", day1(0));
    println!("{:?}", day1(1));
    println!("{:?}", day2());
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
                let mut max_dist_abs = match dir.signum() {
                    1 => size - pos,
                    -1 => pos,
                    _ => panic!(),
                };
                if max_dist_abs == 0 {
                    max_dist_abs = size;
                }
                let actual_dist_abs = i32::min(max_dist_abs, dist_remaining_abs);
                pos += actual_dist_abs * dir.signum();
                pos = pos.rem_euclid(size);
                dist_remaining_abs -= actual_dist_abs;

                if pos == 0 {
                    password += 1;
                }
            }
        }
    }

    password
}

fn day2() -> u64 {
    // just gonna brute force p1 and assume p2 is going to be huge ranges of billions to punish me.

    read_to_string("./src/d2_input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .map(|(min, max)| {
            let mut repeats = 0;
            for id in min..=max {
                let as_str = id.to_string();
                if as_str.len() % 2 == 0 {
                    let mid = as_str.len() / 2;
                    if as_str[..mid] == as_str[mid..] {
                        repeats += 1;
                    }
                }
            }
            repeats
        })
        .sum()
}
