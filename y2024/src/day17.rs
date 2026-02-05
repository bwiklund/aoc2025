use regex::{self, Regex};

#[derive(Debug)]
struct Machine {
    a: i32,
    b: i32,
    c: i32,
    pc: usize,
    program: Vec<i8>,
}

pub fn solve(part: u32) -> i64 {
    let register_re = Regex::new(r"^Register ([A-C]): (\d+)$").unwrap();
    let program_re = Regex::new(r"^Program: ([0-9\,]+)$").unwrap();
    let txt = std::fs::read_to_string("./src/day17_input.txt").unwrap();

    let mut machine = Machine {
        a: 0,
        b: 0,
        c: 0,
        pc: 0,
        program: vec![],
    };

    txt.lines().for_each(|l| {
        if let Some(caps) = register_re.captures(l) {
            let n = caps[2].parse().unwrap();
            match caps[1].to_string().chars().nth(0) {
                Some('A') => machine.a = n,
                Some('B') => machine.b = n,
                Some('C') => machine.c = n,
                _ => panic!("unknown register"),
            }
        } else if let Some(caps) = program_re.captures(l) {
            machine.program = caps[1].split(",").map(|s| s.parse().unwrap()).collect()
        } else if l.is_empty() {
            // ignore
        } else {
            panic!("bad input: {:}", l);
        }
    });

    dbg!(machine);

    match part {
        0 => 0,

        1 => 0,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
