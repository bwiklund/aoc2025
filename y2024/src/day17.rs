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
    /*
    # program "decompiled":
        2,4     b = a % 8            // basically popping the value off a, if a long number were a stack, and b is accum
        1,2     b = b xor 2          // flip 2 bit of accum. accum is still <= 7
        7,5     c = a / (2 pow b)    // c takes some value higher in a stack, basically
        4,3     b = b xor c          // b is xor'd with that
        0,3     a = a / 8            // move to next value in the a "stack"
        1,7     b = b xor 7          // flip lowest 3 bits in b. possibly leaving some higher bits form 2 instructions ago?
        5,5     print(b % 8)         // output low 3 bits of b
        3,0     jnz start            // loop

    # observations:
        - a is effectively a stack of 3 bit numbers. hopefully one that fits in an i64. 63 positive bits would be 21 numbers max, so yes more than we need.
        - a is popped off and then division is used to go to to the next value. b and c never are inputs to it. a is zero when the program is done (for the jnz)
        - c is overwritten each frame and used as a temp var basically, for altering the output from b before OUT
        - so the procedure in english is:

        b = pop 3 bits from a

            // do gross bitwise stuff to b
        b = b xor 2
        c = a / (2 pow b)
        b = b xor c
        print low 3 bits of b, flipped
            // end gross bitwise stuff

        advance a
        loop if a has more

        since we are pulling abitrary random stuff out of the higher bits of a every loop, we could work backwards from the end to make this work, since we know the last output has zero in the upper bits (since the next pass skips the jne)

        (aside: i think the bitwise stuff in the middle is too gross to reason about for a human brain... but i can just test every 3 bit entry in reverse)

        so the procedure could be:
        for each element of the desired program, in reverse:
            test all 7 possible things a could have been at the start, in the low bits
            for each that gives an output we want, (will there be more than one?) move further into the problem (recursively?)

    */

    let machine = parse_input();

    let program_as_i64 = machine
        .program
        .iter()
        .map(|x| (*x) as i64)
        .collect::<Vec<_>>();

    let a_value = rec(&machine, &program_as_i64, 0, 0).expect("Didn't work");

    let mut final_check = machine.clone();
    final_check.a = a_value;
    while final_check.tick() {}
    if final_check.output != program_as_i64 {
        panic!("not a valid solution!");
    }

    a_value
}

fn rec(machine: &Machine, program_as_i64: &Vec<i64>, i: usize, a_accum: i64) -> Option<i64> {
    for n in 0..8 {
        let mut candidate = machine.clone();
        candidate.output = program_as_i64[0..program_as_i64.len() - i - 1]
            .iter()
            .copied()
            .collect();
        let candidate_a = (a_accum << 3) + n;
        candidate.a = candidate_a;
        while candidate.tick() {}
        if candidate.output == *program_as_i64 {
            // dbg!(i, n, a_accum);
            // go deeper back in time
            if i == program_as_i64.len() - 1 {
                return Some(candidate_a); // we're done i think
            } else {
                // recurse because each step has more than one valid solution
                if let Some(res) = rec(&machine, &program_as_i64, i + 1, candidate_a) {
                    return Some(res);
                }
            }
        }
    }
    None
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
        assert_eq!(solve_p2(), 190384609508367);
    }
}
