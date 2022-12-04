use anyhow::{Result, Error};
use std::{str::FromStr};

mod misc; //input utils

type Points = i32;

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
  fn points(self) -> Points {
    match &self {
      Game::Win => 6,
      Game::Tie => 3,
      Game::Loss => 0,
    }
  }
}

impl Turn {
  fn points(self) -> Points {
    match &self {
      Turn::Rock => 1,
      Turn::Paper => 2,
      Turn::Scissors => 3,
    }
  }

  fn play_part_1(self: &Turn, opp: &Turn) -> Game {
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

  fn play_part_2(opp: &Turn, need_to: &Game) -> Turn {
    match (opp, need_to) {
      (Turn::Rock, Game::Loss) => Turn::Scissors,
      (Turn::Rock, Game::Tie) => Turn::Rock,
      (Turn::Rock, Game::Win) => Turn::Paper,

      (Turn::Paper, Game::Loss) => Turn::Rock,
      (Turn::Paper, Game::Tie) => Turn::Paper,
      (Turn::Paper, Game::Win) => Turn::Scissors,

      (Turn::Scissors, Game::Loss) => Turn::Paper,
      (Turn::Scissors, Game::Tie) => Turn::Scissors,
      (Turn::Scissors, Game::Win) => Turn::Rock,
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

impl FromStr for Game {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "X" => Ok(Game::Loss),
      "Y" => Ok(Game::Tie),
      "Z" => Ok(Game::Win),
      _ => Err(()),
    }
  }
}


impl misc::File {
  fn play (&self) -> (Points, Points) {
    self.content
    .trim()
    .lines()
    .map(|turn| turn.split_once(' ' ).unwrap())
    .fold((0, 0), |(part_1, part_2), (opp, you)| {
      let result_1 = Turn::play_part_1(&Turn::from_str(you).unwrap(), &Turn::from_str(opp).unwrap());
      let result_2 = Turn::play_part_2(&Turn::from_str(opp).unwrap(), &Game::from_str(you).unwrap());
      (part_1 + Turn::from_str(you).unwrap().points() + result_1.points(),  part_2 + Game::from_str(you).unwrap().points() + result_2.points())
    })
  }
}

pub fn main() -> Result<(Points, Points), Error> {
  let file = misc::get_file();
  Ok(file.play())
}

#[cfg(test)]
mod tests {
    use super::misc::File;

    #[test]
    fn test_a() {
        let sample_data: File = File::new(String::from("A Y\nB X\nC Z\n"));
        let (part_1, _) = sample_data.play();
        assert_eq!(part_1, 15);
    }

    #[test]
    fn test_b() {
      let sample_data: File = File::new(String::from("A Y\nB X\nC Z\n"));
      let (_, part_2) = sample_data.play();
      assert_eq!(part_2, 12);  
    }
}