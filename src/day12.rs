#![allow(dead_code)]

use std::{iter::repeat_with, ops::RangeInclusive};

#[derive(Debug)]
struct Shape {
    id: u64,
    w: usize,
    h: usize,
    cells: Vec<bool>,
}

impl Shape {
    fn add_line(&mut self, row: Vec<bool>) {
        self.h += 1;
        self.w = row.len();
        self.cells.extend(row);
    }
}

#[derive(Debug)]
struct Region {
    w: usize,
    h: usize,
    shape_counts: Vec<u64>,
}

struct Parser {
    chars: Vec<char>,
    i: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            i: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.i).cloned()
    }

    fn peek_ch(&self, ch: char) -> bool {
        match self.peek() {
            Some(c) => c == ch,
            None => false,
        }
    }

    fn accept(&mut self, ch: char) -> Option<char> {
        let res = self.chars.get(self.i).cloned().filter(|c| *c == ch);
        if res.is_some() {
            self.i += 1;
        }
        res
    }

    fn accept_range(&mut self, range: RangeInclusive<char>) -> Option<char> {
        let ch = self.chars.get(self.i)?;
        match range.contains(ch) {
            true => {
                self.i += 1;
                Some(*ch)
            }
            false => None,
        }
    }

    fn number(&mut self) -> Option<u64> {
        let back = self.i;
        let res = repeat_with(|| self.accept_range('0'..='9'))
            .map_while(|x| x)
            .collect::<String>()
            .parse()
            .ok();
        if res.is_none() {
            self.i = back
        }
        res
    }
}

pub fn solve(part: u32) -> u64 {
    let mut shapes = vec![];
    let mut regions = vec![];

    std::fs::read_to_string("./src/day12_input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            let mut p = Parser::new(l);
            if l.is_empty() {
                return;
            }

            if let Some(n) = p.number() {
                if p.accept('x').is_some() {
                    let w = n as usize;
                    let h = p.number().unwrap() as usize;
                    p.accept(':').unwrap();

                    let mut shape_counts = vec![];
                    while p.peek().is_some() {
                        p.accept(' ').unwrap();
                        shape_counts.push(p.number().unwrap())
                    }
                    regions.push(Region { w, h, shape_counts })
                } else if p.accept(':').is_some() {
                    shapes.push(Shape {
                        id: n,
                        w: 0,
                        h: 0,
                        cells: vec![],
                    });
                } else {
                    panic!();
                }
            } else if p.peek_ch('.') || p.peek_ch('#') {
                let mut row = vec![];
                while let Some(ch) = p.accept('#').or_else(|| p.accept('.')) {
                    row.push(match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    });
                }
                shapes.last_mut().expect("can't be empty").add_line(row);
            } else {
                panic!();
            }
        });

    dbg!(shapes);
    dbg!(regions.iter().take(10).collect::<Vec<_>>());

    match part {
        0 => 0,

        1 => 0,

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        assert_eq!(solve(0), 0);
        // assert_eq!(solve(1), 0);
    }
}
