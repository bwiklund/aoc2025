use regex::{self, Regex};

#[derive(Debug)]
struct Machine {
    a: i32,
    b: i32,
    c: i32,
    pc: usize,
    program: Vec<u8>,
    output: Vec<i32>,
}

impl Machine {
    fn tick(&mut self) -> bool {
        if let Some((inst, op)) = self.read_next_or_halt() {
            match inst {
                // ADV
                0 => self.a = self.a / (1 << self.as_combo(op)),

                // BXL
                1 => self.b = self.b ^ (op as i32),

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

    fn as_combo(&self, op: u8) -> i32 {
        match op {
            0..=3 => op.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("bad combo op: {:}", op),
        }
    }
}

pub fn solve(part: u32) -> String {
    let mut machine = parse_input();

    match part {
        0 => {
            while machine.tick() {}
            machine
                .output
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        }

        1 => "".to_string(),

        _ => unreachable!(),
    }
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
        assert_eq!(solve(0), "2,3,4,7,5,7,3,0,7");
        // assert_eq!(solve(1), 0);
    }
}
