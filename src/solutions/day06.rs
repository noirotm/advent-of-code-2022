use crate::solver::Solver;
use std::collections::BTreeSet;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.bytes().flatten().collect::<Vec<_>>()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        solve_for_size(input, 4)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        solve_for_size(input, 14)
    }
}

fn solve_for_size(input: &[u8], n: usize) -> usize {
    input
        .windows(n)
        .enumerate()
        .find(|&(i, b)| are_all_different(b))
        .map(|(i, _)| i + n)
        .expect("not empty")
}

fn are_all_different(bytes: &[u8]) -> bool {
    BTreeSet::from_iter(bytes).len() == bytes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_part1() {
        let inputs = &[
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        let pb = Problem {};

        for (input, s1) in *inputs {
            let input = pb.parse_input(input.as_bytes());

            assert_eq!(pb.solve_first(&input), s1);
        }
    }

    #[test]
    fn test_problem_part2() {
        let inputs = &[
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        let pb = Problem {};

        for (input, s2) in *inputs {
            let input = pb.parse_input(input.as_bytes());
            assert_eq!(pb.solve_second(&input), s2);
        }
    }
}
