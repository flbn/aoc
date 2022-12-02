use aoc::utils::input;

type Calories = Vec<u32>;

struct File {
  content: String
}

impl File {
  fn new (c: String) -> File {
    Self { content: c }
  }

  fn parse_input_as_u32 (self) -> Calories {
    self.content
      .split("\n\n")
      .map(|elf| elf.lines().map(|x| x.parse::<u32>().unwrap()).sum())
      .collect()
  }
}

fn get_file() -> File {
    let current_file = file!();
    File::new(input::read_file(current_file))
}

pub fn main() {
    let file = get_file();
    let calories: Calories = file.parse_input_as_u32();

    let part_1: u32 = get_most_cal(calories.clone());
    let part_2 = get_top_three(calories);

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

fn get_most_cal(vec: Calories) -> u32 {
  vec.iter().copied().max().unwrap_or_default()
}

fn get_top_three(vec: Calories) -> u32 {
  let mut top = [u32::MIN; 3];
  for calories in vec.iter() {
    let mut calories = *calories;

    for max in top.iter_mut() {
        if calories > *max {
            std::mem::swap(max, &mut calories);
        }
    }
  }

  top.iter().sum::<u32>()
}