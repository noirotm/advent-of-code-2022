use crate::grid::Grid;
use crate::solver::{ReadExt, Solver};
use scan_fmt::scan_fmt;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instr>;
    type Output1 = i64;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let s = compute_states(input);
        [20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|i| s[i - 1] * (i as i64))
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (w, h) = (40, 6);

        let mut screen = Grid::new_with(w, h, Pixel::Dark);
        let screen_as_slice = screen.as_slice_mut();

        let states = compute_states(input);

        for (cycle, &x) in states.iter().enumerate() {
            let c = cycle as i64 % w as i64;
            let x = x % w as i64;
            if [x - 1, x, x + 1].contains(&c) {
                screen_as_slice[cycle] = Pixel::Lit;
            }
        }

        format!("\n{screen}")
    }
}

fn compute_states(program: &[Instr]) -> Vec<i64> {
    program.iter().fold(vec![1], next_step)
}

fn next_step(mut status: Vec<i64>, instr: &Instr) -> Vec<i64> {
    let last = status.last().cloned().unwrap_or(1);
    status.push(last);
    if let Instr::Addx(n) = instr {
        status.push(last + n);
    }
    status
}

#[derive(Debug)]
pub enum Instr {
    Noop,
    Addx(i64),
}

impl FromStr for Instr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(n) = scan_fmt!(s, "addx {}", i64) {
            Self::Addx(n)
        } else {
            Self::Noop
        })
    }
}

#[derive(Clone)]
pub enum Pixel {
    Lit,
    Dark,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Lit => '#',
                Pixel::Dark => '.',
            }
        )
    }
}
