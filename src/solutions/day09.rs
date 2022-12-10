use crate::solver::{ReadExt, Solver};
use anyhow::{anyhow, Error};
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instr>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        solve_for_size(2, input)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        solve_for_size(10, input)
    }
}

fn solve_for_size(n: usize, instrs: &[Instr]) -> usize {
    const ORIGIN: (isize, isize) = (0, 0);
    let mut knots = vec![ORIGIN; n];

    let mut visited = HashSet::new();
    visited.insert(ORIGIN);

    for instr in instrs {
        for _ in 0..instr.n {
            knots[0] = move_head(knots[0], &instr.dir);
            for i in 1..n {
                knots[i] = move_tail(knots[i], knots[i - 1]);
            }
            visited.insert(knots[n - 1]);
        }
    }

    visited.len()
}

fn move_head((x, y): (isize, isize), dir: &Dir) -> (isize, isize) {
    match dir {
        Dir::Up => (x, y + 1),
        Dir::Down => (x, y - 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

fn move_tail((tx, ty): (isize, isize), (hx, hy): (isize, isize)) -> (isize, isize) {
    // close enough, do nothing
    if hx.abs_diff(tx) <= 1 && hy.abs_diff(ty) <= 1 {
        (tx, ty)
    } else {
        let dx = (hx - tx).signum();
        let dy = (hy - ty).signum();
        (tx + dx, ty + dy)
    }
}

#[derive(Debug)]
pub struct Instr {
    dir: Dir,
    n: isize,
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, n) = scan_fmt!(s, "{} {}", String, isize)?;
        Ok(Self {
            dir: dir.parse()?,
            n,
        })
    }
}

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => Err(anyhow!("{s}: invalid direction"))?,
        })
    }
}
