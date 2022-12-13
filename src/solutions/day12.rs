use crate::grid::Grid;
use crate::solver::Solver;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Mark>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).expect("valid grid")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let from = find_origin(input).expect("origin present");
        find_shortest_path_to_exit(input, from)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        (0..input.w)
            .cartesian_product(0..input.h)
            .filter(|coord| is_possible_start(input, coord))
            .map(|coord| find_shortest_path_to_exit(input, coord))
            .min()
            .expect("a minimum")
    }
}

fn is_possible_start(grid: &Grid<Mark>, coord: &(usize, usize)) -> bool {
    matches!(grid.get(coord), Some(Mark::Start | Mark::Elevation(b'a')))
}

fn find_origin(grid: &Grid<Mark>) -> Option<(usize, usize)> {
    (0..grid.w)
        .cartesian_product(0..grid.h)
        .find(|coord| matches!(grid.get(coord), Some(Mark::Start)))
}

fn find_shortest_path_to_exit(grid: &Grid<Mark>, from: (usize, usize)) -> usize {
    // Dijkstra to the rescue
    let mut unvisited = HashSet::new();
    for y in 0..grid.h {
        for x in 0..grid.w {
            unvisited.insert((x, y));
        }
    }

    let mut total_cost = usize::MAX;
    let mut queue = VecDeque::from(vec![(from, 0)]);

    while let Some(((x, y), cost)) = queue.pop_front() {
        let pt = (x, y);
        if !unvisited.contains(&pt) {
            continue;
        }

        unvisited.remove(&pt);

        match grid.get(&pt) {
            Some(current @ (Mark::Start | Mark::Elevation(_))) => {
                let neighbours = grid.neighbours_coords4(pt);
                for n in neighbours {
                    if let Some(m) = grid.get(n) {
                        if current.can_go_to(m) {
                            queue.push_back((n, cost + 1));
                        }
                    }
                }
            }
            Some(Mark::Exit) => {
                // it's over
                total_cost = cost;
                break;
            }
            None => {}
        }
    }

    total_cost
}

pub enum Mark {
    Elevation(u8),
    Start,
    Exit,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mark::Elevation(e) => *e as char,
                Mark::Start => 'S',
                Mark::Exit => 'E',
            }
        )
    }
}

impl Mark {
    fn can_go_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Elevation(a), Self::Elevation(b)) => a > b || a.abs_diff(*b) <= 1,
            (Self::Start, Self::Elevation(b)) => b'a'.abs_diff(*b) <= 1,
            (Self::Elevation(a), Self::Exit) => a.abs_diff(b'z') <= 1,
            _ => false,
        }
    }
}

impl TryFrom<u8> for Mark {
    type Error = Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'a'..=b'z' => Ok(Self::Elevation(b)),
            b'E' => Ok(Self::Exit),
            b'S' => Ok(Self::Start),
            _ => Err(anyhow!("invalid mark")),
        }
    }
}
