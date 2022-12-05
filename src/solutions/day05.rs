use crate::solver::{ReadExt, Solver};
use anyhow::{anyhow, Error};
use scan_fmt::scan_fmt;
use std::collections::VecDeque;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Supply;
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Supply::from_reader(r).expect("correct supply")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut crates = input.crates.clone();
        for i in &input.instructions {
            crates.move_crates(i);
        }

        crates
            .0
            .iter()
            .filter_map(|c| c.last())
            .map(|b| *b as char)
            .collect::<String>()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut crates = input.crates.clone();
        for i in &input.instructions {
            crates.move_crates_advanced(i);
        }

        crates
            .0
            .iter()
            .filter_map(|c| c.last())
            .map(|b| *b as char)
            .collect::<String>()
    }
}

#[derive(Debug)]
pub struct Supply {
    crates: Crates,
    instructions: Vec<Instruction>,
}

impl Supply {
    fn from_reader<R: Read>(r: R) -> Result<Self, Error> {
        let s: Vec<String> = r.split_groups();
        Ok(Self {
            crates: s.get(0).ok_or_else(|| anyhow!("missing crates"))?.parse()?,
            instructions: s
                .get(1)
                .ok_or_else(|| anyhow!("missing instructions"))?
                .as_bytes()
                .split_lines(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Crates(Vec<Vec<u8>>);

impl Crates {
    fn move_crates(&mut self, instr: &Instruction) {
        let (from, to) = (instr.from - 1, instr.to - 1);
        for _ in 0..instr.n {
            let elem = self.0[from].pop().expect("non empty vector");
            self.0[to].push(elem);
        }
    }

    fn move_crates_advanced(&mut self, instr: &Instruction) {
        let (from, to) = (instr.from - 1, instr.to - 1);
        let drain_range = self.0[from].len() - instr.n..self.0[from].len();
        let drained = self.0[from].drain(drain_range).collect::<Vec<_>>();
        self.0[to].extend(drained);
    }
}

impl FromStr for Crates {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = vec![];

        // parse line by line
        'lines: for l in s.lines() {
            let buffer = l.as_bytes();

            // search for letters at each possible location
            'characters: for i in 0..16 {
                let idx = (i * 4) + 1;
                if let Some(b) = buffer.get(idx) {
                    if b.is_ascii_alphabetic() {
                        if crates.len() < i + 1 {
                            crates.resize(i + 1, VecDeque::new());
                        }

                        crates.get_mut(i).expect("correct index").push_front(*b);
                    } else if b.is_ascii_digit() {
                        // digit ? we can return
                        break 'lines;
                    }
                } else {
                    break 'characters;
                }
            }
        }

        // cleanup the vecdeques to make them vecs
        Ok(Crates(crates.into_iter().map(|v| v.into()).collect()))
    }
}

#[derive(Debug)]
pub struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, from, to) = scan_fmt!(s, "move {} from {} to {}", usize, usize, usize)?;
        Ok(Self { n, from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let pb = Problem {};
        let input = pb.parse_input(input.as_bytes());

        assert_eq!(pb.solve_first(&input), "CMZ".to_string());
        assert_eq!(pb.solve_second(&input), "MCD".to_string());
    }
}
