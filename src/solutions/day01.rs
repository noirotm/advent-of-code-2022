use crate::solver::{ReadExt, Solver};
use itertools::Itertools;
use std::convert::Infallible;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Calories>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_groups()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|v| v.sum()).max().unwrap_or_default()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|v| v.sum())
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(3)
            .sum()
    }
}

pub struct Calories(Vec<u64>);

impl FromStr for Calories {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.as_bytes().split_lines()))
    }
}

impl Calories {
    fn sum(&self) -> u64 {
        self.0.iter().sum()
    }
}
