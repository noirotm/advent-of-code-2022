use crate::solver::{ReadExt, Solver};
use scan_fmt::scan_fmt;
use std::collections::BTreeSet;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Pair>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|p| p.contains_other()).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|p| p.overlaps()).count()
    }
}

#[derive(Debug)]
pub struct Pair {
    first: RangeInclusive<u8>,
    second: RangeInclusive<u8>,
}

impl Pair {
    fn contains_other(&self) -> bool {
        (self.first.start() <= self.second.start() && self.first.end() >= self.second.end())
            || (self.first.start() >= self.second.start() && self.first.end() <= self.second.end())
    }

    fn overlaps(&self) -> bool {
        let f = BTreeSet::from_iter(self.first.clone());
        let s = BTreeSet::from_iter(self.second.clone());
        f.intersection(&s).count() > 0
    }
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, c, d) = scan_fmt!(s, "{}-{},{}-{}", u8, u8, u8, u8).map_err(|_| "error")?;
        Ok(Self {
            first: a..=b,
            second: c..=d,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        assert!(Pair {
            first: 1..=2,
            second: 1..=2,
        }
        .contains_other());

        assert!(Pair {
            first: 1..=2,
            second: 0..=20,
        }
        .contains_other());

        assert!(!Pair {
            first: 1..=2,
            second: 2..=20,
        }
        .contains_other());
    }

    #[test]
    fn test_pair_overlap() {
        assert!(Pair {
            first: 1..=2,
            second: 1..=2,
        }
        .overlaps());

        assert!(Pair {
            first: 1..=2,
            second: 0..=20,
        }
        .overlaps());

        assert!(Pair {
            first: 1..=2,
            second: 2..=20,
        }
        .overlaps());
    }

    #[test]
    fn test_problem() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let pb = Problem {};
        let input = pb.parse_input(input.as_bytes());

        assert_eq!(pb.solve_first(&input), 2);
        assert_eq!(pb.solve_second(&input), 4);
    }
}
