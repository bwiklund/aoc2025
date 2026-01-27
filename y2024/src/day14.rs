use regex::Regex;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn tick(&mut self) {
        self.x = (self.x + self.vx).rem_euclid(101);
        self.y = (self.y + self.vy).rem_euclid(103);
    }
}

pub fn solve(part: u32) -> i64 {
    let re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = std::fs::read_to_string("./src/day14_input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();

            Robot {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                vx: caps[3].parse().unwrap(),
                vy: caps[4].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    match part {
        0 => {
            for _ in 0..100 {
                robots.iter_mut().for_each(|r| r.tick());
            }
            let q1 = robots.iter().filter(|r| r.x < 50 && r.y < 51).count() as i64;
            let q2 = robots.iter().filter(|r| r.x > 50 && r.y < 51).count() as i64;
            let q3 = robots.iter().filter(|r| r.x < 50 && r.y > 51).count() as i64;
            let q4 = robots.iter().filter(|r| r.x > 50 && r.y > 51).count() as i64;
            q1 * q2 * q3 * q4
        }

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
