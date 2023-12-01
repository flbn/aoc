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

  fn parse_input_as_nums (&self) -> Vec<Nums> {
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

 fn parse_input_as_nums_with_str(&self) -> Vec<Nums> {
        let mut results = Vec::new();
        for line in self.content.lines() {
            let mut nums = Vec::new();
            let mut current_word = String::new();

            for c in line.chars() {
                if c.is_ascii_digit() {
                    nums.push(c as u32 - '0' as u32);
                    current_word.clear();
                    break;
                } else if c.is_ascii_alphabetic() {
                    current_word.push(c);
                    if let Some(num) = word_to_number(&current_word, false) {
                        nums.push(num);
                        current_word.clear();
                        break;
                    }
                }
            }

            for c in line.chars().rev() {
              if c.is_ascii_digit() {
                  nums.push(c as u32 - '0' as u32);
                  current_word.clear();
                  break;
              } else if c.is_ascii_alphabetic() {
                  current_word.push(c);
                  if let Some(num) = word_to_number(&current_word, true) {
                      nums.push(num);
                      current_word.clear();
                      break;
                  }
              }
            }

          
            results.push(nums);
        }
        results
    }
}

fn word_to_number(word: &str, rev: bool) -> Option<u32> {
  if word.contains("one") && rev == false { return Some(1) }
  else if word.contains("eno") && rev == true { return Some(1) }
  else if word.contains("two") && rev == false { return Some(2) }
  else if word.contains("owt") && rev == true { return Some(2) }
  else if word.contains("three") && rev == false { return Some(3) }
  else if word.contains("eerht") && rev == true { return Some(3) }
  else if word.contains("four") && rev == false { return Some(4) }
  else if word.contains("ruof") && rev == true { return Some(4) }
  else if word.contains("five") && rev == false { return Some(5) }
  else if word.contains("evif") && rev == true { return Some(5) }
  else if word.contains("six") && rev == false { return Some(6) }
  else if word.contains("xis") && rev == true { return Some(6) }
  else if word.contains("seven") && rev == false { return Some(7) }
  else if word.contains("neves") && rev == true { return Some(7) }
  else if word.contains("eight") && rev == false { return Some(8) }
  else if word.contains("thgie") && rev == true { return Some(8) }
  else if word.contains("nine") && rev == false { return Some(9) }
  else if word.contains("enin") && rev == true { return Some(9) }

  None
}

fn get_file() -> File {
  let current_file = file!();
  File::new(input::read_file(current_file))
}

pub fn main() -> Result<(Num, Num), Error> {
  let file = get_file();
  let part_1_nums = file.parse_input_as_nums();
  let part_2_nums = file.parse_input_as_nums_with_str();

  Ok((
    part_1(part_1_nums),part_2(part_2_nums)
  ))
}

pub fn part_1(nums: Vec<Nums>) -> u32 {
  nums.into_iter()
    .map(|line| {
      match line.as_slice() {
        [first, _rest @ .., last] => first * 10 + last,
        [single] => single * 11,
        _ => 0,
      }
    })
    .sum()
}

pub fn part_2(nums: Vec<Nums>) -> u32 {
   nums.into_iter()
    .map(|line| {
      match line.as_slice() {
        [first, _rest @ .., last] => first * 10 + last,
        [single] => single * 11,
        _ => 0,
      }
    })
    .sum()
}
