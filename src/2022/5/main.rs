use anyhow::{Result, Error};
use regex::Regex;

mod misc; //input utils

#[derive(Debug)]
pub struct Crate {
  val: usize,
  origin: usize,
  dest: usize,
}

impl Crate {
  fn new(v: usize, o: usize, d: usize) -> Crate {
    Self { val: v, origin: o, dest: d }
  }
}

pub fn a(mut crates: Vec<Vec<char>>, instructions: Vec<Crate>) -> String { 
  println!("Crates: {:?}", crates);
  println!("Cmds: {:?}", instructions);

  for instruction in instructions {
      for _ in  0.. instruction.val {
          // println!("Move from {} {:?} to {} {:?} {}", 
          //     cmd.from, crates[cmd.from - 1], 
          //     cmd.to, crates[cmd.to-1],
          //     cmd.count );
          let v = crates[instruction.origin - 1].pop().expect("Invalid move");
          crates[instruction.dest - 1].push(v);
      }
  }

  let res: String = crates.iter().map(|stack| stack[stack.len()-1]).collect();
  res
}

fn parse_input_as_crates (i: &str) -> (Vec<Vec<char>>, Vec<Crate>) {
  let regex_for_instruction = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("failed to compile a regex for the instruction");
  
  let mut instructions = vec!();
  let mut crates = vec!();

  for line in i.lines() {
    if line.contains('[') {
      let crates_found = (line.len() / 4) + 1; // [Z] [M] [P] -> 11 len -> 11 / 4 = 2 -> 2 + 1 -> 3
      while crates.len() < crates_found {
        crates.push(vec!());
      }
      for (i, c) in line.chars().enumerate() {
        if c.is_ascii_alphabetic() {
          crates[i / 4].insert(0, c)
        }
      }
    } else if regex_for_instruction.is_match(line) {
      for cap in regex_for_instruction.captures_iter(line) {
        instructions.push(
          Crate::new(
            cap[1].parse().expect("failed to parse this string slice into value usize")
            , cap[2].parse().expect("failed to parse this string slice into origin usize")
            , cap[3].parse().expect("failed to parse this string slice into destination usize")
          )
        );
      }
    }
  }

  (crates, instructions)
}

pub fn main() -> Result<String, Error> {
  let file = misc::get_file();

  let (crates, instructions) = parse_input_as_crates(&file.content);
  let part_a = a(crates, instructions);


  Ok(part_a)
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_DATA: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    
  #[test]
  fn test_a() {
    let (crates, instructions) = parse_input_as_crates(SAMPLE_DATA);
    let part_a = a(crates, instructions);
    assert_eq!(part_a, "CMZ");
  }

  // #[test]
  // fn test_b() {
  //   assert_eq!();  
  // }
}