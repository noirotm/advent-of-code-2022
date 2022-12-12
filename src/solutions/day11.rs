use crate::solver::{ReadExt, Solver};
use anyhow::Error;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::btree_map::BTreeMap;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Monkey>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_groups()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut monkeys = input.clone();

        for _ in 0..20 {
            evaluate_round(&mut monkeys, |v| v / 3);
        }

        monkeys
            .iter()
            .map(|m| m.total_examined)
            .sorted_unstable()
            .rev()
            .take(2)
            .product()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut monkeys = input.clone();

        // find LCM for all monkeys (they're prime, so it's just the product)
        let lcm = monkeys.iter().map(|m| m.test_divisible_by).product::<u64>();

        // wrapping each integer to its modulo works, because all the operations either
        // preserve the modulo via multiplying, or add a fixed amount so modulo will stay the same
        for _ in 0..10000 {
            evaluate_round(&mut monkeys, |v| v % lcm);
        }

        monkeys
            .iter()
            .map(|m| m.total_examined)
            .sorted_unstable()
            .rev()
            .take(2)
            .product()
    }
}

fn evaluate_turn<F>(monkeys: &mut [Monkey], i: usize, value_control: F)
where
    F: Fn(u64) -> u64,
{
    let m = &monkeys[i];
    let mut moves = BTreeMap::new();

    for (idx, item) in m.items.iter().enumerate() {
        let worry = m.operation.apply(*item);
        let worry = value_control(worry);

        let next_monkey = if (worry % m.test_divisible_by) == 0 {
            m.next_monkey.0
        } else {
            m.next_monkey.1
        };
        moves.insert(idx, (next_monkey, worry));
    }

    for (&item_idx, &(next_monkey, worry)) in moves.iter().rev() {
        monkeys[i].total_examined += 1;
        let _ = monkeys[i].items.remove(item_idx);
        monkeys[next_monkey].items.push(worry);
    }
}

fn evaluate_round<F>(monkeys: &mut [Monkey], value_control: F)
where
    F: Fn(u64) -> u64,
{
    for i in 0..monkeys.len() {
        evaluate_turn(monkeys, i, &value_control);
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    next_monkey: (usize, usize),
    total_examined: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Self {
            items: vec![],
            operation: Operation::Square,
            test_divisible_by: 0,
            next_monkey: (0, 0),
            total_examined: 0,
        };

        for line in s.lines() {
            if let Ok(items_str) = scan_fmt!(line, "  Starting items: {[0-9, ]}", String) {
                m.items
                    .extend(items_str.split(", ").flat_map(|s| s.parse::<u64>()));
            } else if let Ok(n) = scan_fmt!(line, "  Operation: new = old * {}", u64) {
                m.operation = Operation::Mul(n);
            } else if let Ok(n) = scan_fmt!(line, "  Operation: new = old + {}", u64) {
                m.operation = Operation::Add(n);
            } else if let Ok(n) = scan_fmt!(line, "  Test: divisible by {}", u64) {
                m.test_divisible_by = n;
            } else if let Ok(n) = scan_fmt!(line, "    If true: throw to monkey {}", usize) {
                m.next_monkey.0 = n;
            } else if let Ok(n) = scan_fmt!(line, "    If false: throw to monkey {}", usize) {
                m.next_monkey.1 = n;
            }
        }

        Ok(m)
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply(&self, val: u64) -> u64 {
        match self {
            Operation::Add(n) => val + *n,
            Operation::Mul(n) => val * *n,
            Operation::Square => val * val,
        }
    }
}
