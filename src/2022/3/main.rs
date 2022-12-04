use anyhow::{Result, Error};

mod misc; //input utils

pub fn main() -> Result<(u32, u32), Error> {
  let file = misc::get_file();
  let a: u32 = file.part_1();
  let b: u32 = file.part_2();
  Ok((a, b))
}

mod tests {
  #[test]
  fn test_1() {
      use crate::twenty_twenty_two::three::misc::File;
      let sample_data = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
      let file = File::new(sample_data); 
      let part_1 = file.part_1();
      assert_eq!(part_1, 157);
  }

  #[test]
  fn test_2() {
    use crate::twenty_twenty_two::three::misc::File;
    let sample_data = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
    let file = File::new(sample_data); 
    let part_2 = file.part_2();
    assert_eq!(part_2, 70);
  }
}