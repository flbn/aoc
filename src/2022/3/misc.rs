use aoc::utils::input;

pub struct File {
  pub content: String
}

impl File {
  pub fn new (c: String) -> File {
    Self { content: c }
  }

  pub fn part_1 (&self) -> u32 {
  let mut result = 0;
  let split: Vec<&str> = self.content.split('\n').into_iter().collect();
  for line in split {
    let len = line.len();
    let half = line.split_at(len/2);
    let chunk: Vec<char> = half.0.chars().filter(|a| half.1.contains(*a)).collect();
    let priority = match chunk[0].is_uppercase() {
        true => chunk[0] as u32 - 38,
        false => chunk[0] as u32 - 96
    };
    
    result += priority;
  }
  result
  }

  pub fn part_2 (&self) -> u32 {
    let mut result = 0;
    let split: Vec<&str> = self.content.split('\n').into_iter().collect();
    for line in split.chunks(3) {
      let chunk: Vec<char> = line[0]
          .chars()
          .filter(|a| line[1].contains(*a))
          .filter(|a| line[2].contains(*a))
          .collect();
      let priority = match chunk[0].is_uppercase() {
          true => chunk[0] as u32 - 38,
          false => chunk[0] as u32 - 96,
      };
      result += priority;
    }
    result
  }
}

pub fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}