use anyhow::{Result, Error};
use std::ops::{RangeInclusive};

use aoc::utils::input;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    IResult,
    multi::separated_list1,
    sequence::separated_pair,
};

type ID = u32;
type IdRange = RangeInclusive<ID>;

fn sections(input: &str) -> IResult<&str, IdRange> {
  // nom will output a tuple with the input as one arm and the desired output as the other
  // according to this pattern -> complete::u32, followed by the '-' char, follwed by another complete::u32
  // the output is a u32 for the start of range and a u32 for the end of the range
  let (input, (start, end)) = separated_pair(
      complete::u32,
      tag("-"),
      complete::u32,
  )(input)?;

  Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, (IdRange, IdRange)> {
  // same deal,
  // according to this pattern -> the outputted range of a section (complete::u32..=complete::u32), followed by a ',' char, followed by the the outputted range of another section (complete::u32..=complete::u32
  // the left arm is the range for one elf and the right arm is the range for another elf
  let (input, (start, end)) = separated_pair(
    sections,
    tag(","),
    sections
  )(input)?;

  Ok((input, (start, end)))
}

fn section_assignments(input: &str) -> IResult<&str, Vec<(IdRange, IdRange)>> {
  // nom is parsing one line at time
  let (input, ranges) = separated_list1(newline, line)(input)?;
  Ok((input, ranges))
}

pub fn process_part1(input: &str) -> String {
  // a vec of every line represented as inclusive ranges of u32 values
  let (_, assignments) = section_assignments(input).expect("failed to parse input as inclusive ranges");
  let result = assignments
      .iter()
      .filter(|(a, b)| {
          let a_contains_b = a
              .clone()
              .into_iter()
              .all(|num| b.contains(&num));
          let b_contains_a = b
              .clone()
              .into_iter()
              .all(|num| a.contains(&num));
          a_contains_b || b_contains_a
      })
      .count();
  result.to_string()
}

pub fn process_part2(input: &str) -> String {
  // a vec of every line represented as inclusive ranges of u32 values
  let (_, assignments) = section_assignments(input).expect("failed to parse input as inclusive ranges");
  let result = assignments
      .iter()
      .filter(|(a, b)| {
          let a_contains_b = a
              .clone()
              .into_iter()
              .any(|num| b.contains(&num));
          let b_contains_a = b
              .clone()
              .into_iter()
              .any(|num| a.contains(&num));
          a_contains_b || b_contains_a
      })
      .count();
  result.to_string()
}

pub fn main() -> Result<(String, String), Error> {
  let file = input::read_file(file!());
  Ok((
    process_part1(file.as_str()),
    process_part2(file.as_str())
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

    const SAMPLE_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
  
  #[test]
  fn test_a() {
    let a = process_part1(SAMPLE_DATA);
      assert_eq!(a, "2");
    }

    #[test]
    fn test_b() {
      let b = process_part2(SAMPLE_DATA);
      assert_eq!(b, "4");
    }
}