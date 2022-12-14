use anyhow::{Result, Error};
use aoc::utils::input;

type Calorie = u32;
type Calories = Vec<Calorie>;
struct File {
  content: String
}

impl File {
  fn new (c: String) -> File {
    Self { content: c }
  }

  fn parse_input_as_calories (self) -> Calories {
    self.content
      .split("\n\n")
      .map(|elf| elf.lines().map(|x| x.parse::<Calorie>().expect("failed to parse elf calorie file input as a u32")).sum())
      .collect()
  }
}

fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}

pub fn main() -> Result<(Calorie, Calorie), Error> {
  let file = get_file();
  let calories: Calories = file.parse_input_as_calories();

  Ok((get_most_cal(&calories), get_top_three(&calories)))
}

fn get_most_cal(vec: &Calories) -> Calorie {
  vec.iter().copied().max().unwrap_or_default()
}

fn get_top_three(vec: &Calories) -> Calorie {
  let mut top = [Calorie::MIN; 3];
  for calories in vec.iter() {
    let mut calories = *calories;

    for max in top.iter_mut() {
        if calories > *max {
            std::mem::swap(max, &mut calories);
        }
    }
  }

  top.iter().sum::<Calorie>()
}