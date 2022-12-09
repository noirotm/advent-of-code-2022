use crate::grid::Grid;
use crate::solver::Solver;
use itertools::Itertools;
use std::io::Read;
use std::ops::Range;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        (0..input.w)
            .cartesian_product(0..input.h)
            .filter(|&c| !is_tree_hidden(input, c))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        (0..input.w)
            .cartesian_product(0..input.h)
            .map(|c| tree_scenic_score(input, c))
            .max()
            .unwrap_or_default()
    }
}

fn neighbour_ranges(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> (Range<usize>, Range<usize>, Range<usize>, Range<usize>) {
    ((0..x), (x + 1..w), (0..y), (y + 1..h))
}

fn is_tree_hidden(g: &Grid<u8>, (x, y): (usize, usize)) -> bool {
    let current = g.get((x, y)).expect("valid coord");

    let (left, right, top, down) = neighbour_ranges(x, y, g.w, g.h);
    let l = left
        .into_iter()
        .filter_map(|rx| g.get((rx, y)))
        .any(|v| v >= current);
    let r = right
        .into_iter()
        .filter_map(|rx| g.get((rx, y)))
        .any(|v| v >= current);
    let t = top
        .into_iter()
        .filter_map(|ry| g.get((x, ry)))
        .any(|v| v >= current);
    let d = down
        .into_iter()
        .filter_map(|ry| g.get((x, ry)))
        .any(|v| v >= current);

    l && r && t && d
}

fn tree_scenic_score(g: &Grid<u8>, (x, y): (usize, usize)) -> usize {
    let current = g.get((x, y)).expect("valid coord");

    let (left, right, top, down) = neighbour_ranges(x, y, g.w, g.h);
    let (ll, rl, tl, dl) = (left.len(), right.len(), top.len(), down.len());

    let l = left
        .into_iter()
        .rev()
        .position(|rx| g.get((rx, y)).unwrap_or(&0) >= current)
        .map(|n| n + 1)
        .unwrap_or(ll);
    let r = right
        .into_iter()
        .position(|rx| g.get((rx, y)).unwrap_or(&0) >= current)
        .map(|n| n + 1)
        .unwrap_or(rl);
    let t = top
        .into_iter()
        .rev()
        .position(|ry| g.get((x, ry)).unwrap_or(&0) >= current)
        .map(|n| n + 1)
        .unwrap_or(tl);
    let d = down
        .into_iter()
        .position(|ry| g.get((x, ry)).unwrap_or(&0) >= current)
        .map(|n| n + 1)
        .unwrap_or(dl);

    l * r * t * d
}
