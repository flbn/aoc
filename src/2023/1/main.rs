use anyhow::{Result, Error};
use aoc::utils::input;

type Num = u32;
type Nums = Vec<Num>;
struct File {
  content: String
}

impl File {
  fn new (c: String) -> File {
    Self { content: c }
  }

  fn parse_input_as_nums (self) -> Vec<Nums> {
    self.content
      .lines()
      .map(|line|
        line
          .chars()
          .filter(|char| char.is_ascii_digit())
          .map(|c| c as u32 - '0' as u32)
          .collect::<Vec<u32>>()
        )
      .collect()
  }
}

fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}

pub fn main() -> Result<(Num, Num), Error> {
  let file = get_file();
  let nums: Vec<Nums> = file.parse_input_as_nums();

  Ok((part_1(nums), 1))
}

pub fn part_1(nums: Vec<Nums>) -> u32 {
  let sum: u32 = nums.into_iter()
    .map(|line| {
      match line.as_slice() {
        [first, _rest @ .., last] => first * 10 + last,
        [single] => single * 11,
        _ => 0,
      }
    })
    .sum();

  sum
}


