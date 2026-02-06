use regex::{self, Regex};

#[derive(Debug, Clone)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
    program: Vec<u8>,
    output: Vec<i64>,
}

impl Machine {
    fn tick(&mut self) -> bool {
        if let Some((inst, op)) = self.read_next_or_halt() {
            match inst {
                // ADV
                0 => self.a = self.a / (1 << self.as_combo(op)),

                // BXL
                1 => self.b = self.b ^ (op as i64),

                // BST
                2 => self.b = self.as_combo(op) % 8,

                // JNZ
                3 => {
                    if self.a != 0 {
                        self.pc = op as usize;
                    }
                }

                // BXC
                4 => self.b = self.b ^ self.c,

                // OUT
                5 => self.output.push(self.as_combo(op) % 8),

                // BDV
                6 => self.b = self.a / (1 << self.as_combo(op)),

                // CDV
                7 => self.c = self.a / (1 << self.as_combo(op)),

                // unknown op
                _ => panic!("Unknown op {:} at {:}.", op, self.pc),
            }
            return true;
        } else {
            return false;
        }
    }

    fn read_next_or_halt(&mut self) -> Option<(u8, u8)> {
        if self.pc + 1 >= self.program.len() {
            return None;
        }
        let inst = self.program[self.pc];
        let op = self.program[self.pc + 1];
        self.pc += 2;
        Some((inst, op))
    }

    fn as_combo(&self, op: u8) -> i64 {
        match op {
            0..=3 => op.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("bad combo op: {:}", op),
        }
    }
}

pub fn solve_p1() -> String {
    let mut machine = parse_input();
    while machine.tick() {}
    machine
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn solve_p2() -> i64 {
    let machine = parse_input();

    let program_as_i64 = machine
        .program
        .iter()
        .map(|x| (*x) as i64)
        .collect::<Vec<_>>();

    for n in 0.. {
        let mut candidate = machine.clone();
        candidate.a = n;
        if n % 10000000 == 0 {
            dbg!(n);
        }

        let mut early_exit = false;
        while candidate.tick() {
            let len = candidate.output.len();
            if len > program_as_i64.len() {
                early_exit = true;
                break;
            }
            if len > 0 && candidate.output[len - 1] != program_as_i64[len - 1] {
                early_exit = true;
                break;
            }
        }
        if early_exit {
            continue;
        }
        if candidate.output == program_as_i64 {
            return n;
        }
    }
    unreachable!("unless i made a mistake")
}

fn parse_input() -> Machine {
    let register_re = Regex::new(r"^Register ([A-C]): (\d+)$").unwrap();
    let program_re = Regex::new(r"^Program: ([0-9\,]+)$").unwrap();
    let txt = std::fs::read_to_string("./src/day17_input.txt").unwrap();

    let mut machine = Machine {
        a: 0,
        b: 0,
        c: 0,
        pc: 0,
        program: vec![],
        output: vec![],
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
    machine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17() {
        assert_eq!(solve_p1(), "2,3,4,7,5,7,3,0,7");
        assert_eq!(solve_p2(), 0);
    }
}
