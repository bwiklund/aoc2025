use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

pub fn solve(part: u32) -> i64 {
    let re = Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut machines = std::fs::read_to_string("./src/day13_input.txt")
        .unwrap()
        .split("\n\n")
        .map(|str| {
            let caps = re.captures(str).unwrap();
            Machine {
                a: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                b: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
                prize: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    match part {
        0 => machines.iter().map(|m| get_presses(m, true)).sum(),

        1 => {
            let extra = 10000000000000.0;
            machines.iter_mut().for_each(|m| {
                m.prize.0 += extra;
                m.prize.1 += extra;
            });
            machines.iter().map(|m| get_presses(m, false)).sum()
        }

        _ => unreachable!(),
    }
}

fn get_presses(m: &Machine, limit_100: bool) -> i64 {
    // doing linear algebra here by hand because it's just 2 rows...
    // eliminate 1,2
    let sub = m.a.1 / m.a.0;
    let mut rref = (
        m.a.0,
        m.b.0,
        m.prize.0,
        m.a.1 - m.a.0 * sub,
        m.b.1 - m.b.0 * sub,
        m.prize.1 - m.prize.0 * sub,
    );

    let sub = rref.1 / rref.4;
    rref.0 -= rref.3 * sub;
    rref.1 -= rref.4 * sub;
    rref.2 -= rref.5 * sub;

    let scale = rref.0;
    rref.0 /= scale;
    rref.1 /= scale;
    rref.2 /= scale;

    let scale = rref.4;
    rref.3 /= scale;
    rref.4 /= scale;
    rref.5 /= scale;

    fn round_close(n: f64) -> f64 {
        if (n - n.round()).abs() < 0.001 {
            if n.round().abs() == 0.0 {
                return 0.0;
            }
            return n.round();
        } else {
            return n;
        }
    }

    fn is_int(n: f64) -> bool {
        return n.round() == n;
    }

    rref.0 = round_close(rref.0);
    rref.1 = round_close(rref.1);
    rref.2 = round_close(rref.2);
    rref.3 = round_close(rref.3);
    rref.4 = round_close(rref.4);
    rref.5 = round_close(rref.5);

    if is_int(rref.2) && is_int(rref.5) {
        let a_pushes = rref.2 as i64;
        let b_pushes = rref.5 as i64;
        if limit_100 && (a_pushes > 100 || b_pushes > 100) {
            return 0;
        }
        return a_pushes * 3 + b_pushes * 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13() {
        assert_eq!(solve(0), 39996);
        assert_eq!(solve(1), 0);
    }
}
