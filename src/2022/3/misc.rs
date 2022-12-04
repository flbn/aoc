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
  for item in split {
    let len = item.len();
    let first_part = item.split_at(len/2);
    let thing: Vec<char> = first_part.0.chars().filter(|a| first_part.1.contains(*a)).collect();
    let parsed = match thing[0].is_uppercase() {
        true => thing[0] as u32 - 38,
        false => thing[0] as u32 - 96
    };
    
    result += parsed;
  }
  result
  }

  pub fn part_2 (&self) -> u32 {
    let mut result = 0;
    let split: Vec<&str> = self.content.split('\n').into_iter().collect();
    for item in split.chunks(3) {
      let thing: Vec<char> = item[0]
          .chars()
          .filter(|a| item[1].contains(*a))
          .filter(|a| item[2].contains(*a))
          .collect();
      let parsed = match thing[0].is_uppercase() {
          true => thing[0] as u32 - 38,
          false => thing[0] as u32 - 96,
      };
      result += parsed;
    }
    result
  }
}

pub fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}