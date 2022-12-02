use aoc::utils::input;

pub struct File {
  pub content: String
}

impl File {
  fn new (c: String) -> File {
    Self { content: c }
  }
}

pub fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}