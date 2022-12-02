use crate::solver::{ReadExt, Solver};
use std::cmp::Ordering;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Round>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|r| r.score()).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|r| r.score2()).sum()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd<Self> for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Self::Rock, &Self::Paper) => Ordering::Less,
            (&Self::Rock, &Self::Scissors) => Ordering::Greater,
            (&Self::Paper, &Self::Rock) => Ordering::Greater,
            (&Self::Paper, &Self::Scissors) => Ordering::Less,
            (&Self::Scissors, &Self::Rock) => Ordering::Less,
            (&Self::Scissors, &Self::Paper) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl FromStr for RPS {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err("invalid input"),
        }
    }
}

pub enum Intent {
    Lose,
    Draw,
    Win,
}

impl Intent {
    fn as_rps(&self) -> RPS {
        match self {
            Intent::Lose => RPS::Rock,
            Intent::Draw => RPS::Paper,
            Intent::Win => RPS::Scissors,
        }
    }
}

impl FromStr for Intent {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("invalid input"),
        }
    }
}

pub struct Round {
    opponent: RPS,
    player: Intent,
}

impl Round {
    fn score(&self) -> u32 {
        self.points(&self.player.as_rps())
    }

    fn score2(&self) -> u32 {
        let needed_move = match (&self.player, &self.opponent) {
            (Intent::Win, RPS::Rock) => RPS::Paper,
            (Intent::Win, RPS::Paper) => RPS::Scissors,
            (Intent::Win, RPS::Scissors) => RPS::Rock,
            (Intent::Lose, RPS::Rock) => RPS::Scissors,
            (Intent::Lose, RPS::Paper) => RPS::Rock,
            (Intent::Lose, RPS::Scissors) => RPS::Paper,
            (Intent::Draw, rps) => rps.clone(),
        };
        self.points(&needed_move)
    }

    fn points(&self, player: &RPS) -> u32 {
        let choice_points = match player {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let score = match player.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        choice_points + score
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(' ');
        Ok(Self {
            opponent: spl.next().ok_or("missing field")?.parse()?,
            player: spl.next().ok_or("missing field")?.parse()?,
        })
    }
}
