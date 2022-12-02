use anyhow::{Result, Error};
use std::{str::FromStr, str::Split};

mod misc; //input utils

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Game {
    Win,
    Tie,
    Loss,
}

impl Game {
    fn points(self) -> i32 {
        match &self {
          Game::Win => 6,
          Game::Tie => 3,
          Game::Loss => 0,
        }
    }
}

impl Turn {
    fn points(self) -> i32 {
        match &self {
          Turn::Rock => 1,
          Turn::Paper => 2,
          Turn::Scissors => 3,
        }
    }

    fn play(self: &Turn, opp: &Turn) -> Game {
      match (opp, self) {
        (Turn::Rock, Turn::Rock) => Game::Tie,
        (Turn::Rock, Turn::Paper) => Game::Win,
        (Turn::Rock, Turn::Scissors) => Game::Loss,

        (Turn::Paper, Turn::Rock) => Game::Loss,
        (Turn::Paper, Turn::Paper) => Game::Tie,
        (Turn::Paper, Turn::Scissors) => Game::Win,

        (Turn::Scissors, Turn::Rock) => Game::Win,
        (Turn::Scissors, Turn::Paper) => Game::Loss,
        (Turn::Scissors, Turn::Scissors) => Game::Tie,
      }
    }

}

impl FromStr for Turn {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "A" | "X" => Ok(Turn::Rock),
          "B" | "Y" => Ok(Turn::Paper),
          "C" | "Z" => Ok(Turn::Scissors),
          _ => Err(()),
      }
  }
}

impl misc::File {
  fn part_1 (&self) -> i32 {
    self.content
    .trim()
    .lines()
    .map(|turn| turn.split_once(' ' ).unwrap())
    .fold(0, |total_points, (opp, you)| {
      let opp = Turn::from_str(opp).unwrap();
      let you = Turn::from_str(you).unwrap();
      let result = Turn::play(&you, &opp);
      total_points + you.points() + result.points()
    })
  }

  fn part_2 (&self) -> i32 {
    1 + 4
  }
}

pub fn main() -> Result<(i32, i32), Error> {
    let file = misc::get_file();

    Ok((file.part_1(), file.part_2()))
}