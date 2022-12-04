use anyhow::{Result, Error};
use std::{str::FromStr, str::Split};

mod misc; //input utils

pub fn main() -> Result<(Points, Points), Error> {
  let file = misc::get_file();
  Ok()
}

#[cfg(test)]
mod tests {
    use super::misc::File;

    #[test]
    fn test_a() {
        assert_eq!();
    }

    #[test]
    fn test_b() {
      // assert_eq!);  
    }
}