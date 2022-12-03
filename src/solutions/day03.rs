use crate::solver::{ReadExt, Solver};
use std::collections::BTreeSet;
use std::convert::Infallible;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<RuckSack>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|r| r.find_item_in_left_and_right())
            .map(|i| i.priority())
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        groups(input)
            .into_iter()
            .filter_map(find_common_in_group)
            .map(|i| i.priority())
            .sum()
    }
}

fn groups(sacks: &[RuckSack]) -> Vec<&[RuckSack]> {
    sacks.chunks(3).collect()
}

fn find_common_in_group(group: &[RuckSack]) -> Option<Item> {
    let mut it = group.iter();
    let f_items = BTreeSet::from_iter(it.next()?.0.iter());
    let s_items = BTreeSet::from_iter(it.next()?.0.iter());
    let t_items = BTreeSet::from_iter(it.next()?.0.iter());
    let dup_fs = f_items
        .intersection(&s_items)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut dup_fst = t_items.intersection(&dup_fs);

    dup_fst.next().cloned().cloned()
}

pub struct RuckSack(Vec<Item>);

impl RuckSack {
    fn find_item_in_left_and_right(&self) -> Option<Item> {
        let (left, right) = self.0.split_at(self.0.len() / 2);
        debug_assert_eq!(left.len(), right.len());

        let left_items = BTreeSet::from_iter(left);
        let right_items = BTreeSet::from_iter(right);
        let mut dup = left_items.intersection(&right_items);

        dup.next().cloned().cloned()
    }
}

impl FromStr for RuckSack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.as_bytes().iter().map(|b| Item(*b)).collect()))
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Item(u8);

impl Item {
    fn priority(&self) -> u64 {
        (match self.0 {
            b'a'..=b'z' => self.0 - b'a' + 1,
            b'A'..=b'Z' => self.0 - b'A' + 27,
            _ => 0,
        }) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item(b'a').priority(), 1);
        assert_eq!(Item(b'A').priority(), 27);
    }

    #[test]
    fn test_problem() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let pb = Problem {};
        let input = pb.parse_input(input.as_bytes());
        assert_eq!(pb.solve_first(&input), 157);
        assert_eq!(pb.solve_second(&input), 70);
    }
}
