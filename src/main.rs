use std::fs::read_to_string;

fn main() {
    println!("AOC2025 Output:");

    println!("{0}", day1(0));
    println!("{0}", day1(1));
}

fn day1(part: u32) -> i32 {
    let input_str = read_to_string("./src/d1_input.txt").unwrap();

    let directions: Vec<i32> = input_str
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
        .collect();

    let size = 100;
    let mut pos = 50;
    let mut password = 0;
    for dir in directions {
        let new_pos_unclamped = pos + dir;

        pos = new_pos_unclamped.rem_euclid(size);
        if pos == 0 {
            password += 1;
        }

        if part == 1 {
            // count how many times it passed zero as well
            password += dir.abs() / size;
        }
    }

    password
}
