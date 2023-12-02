use anyhow::{Error, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

mod misc;

#[derive(Debug)]
struct Block {
    red: u8,
    blue: u8,
    green: u8,
}

impl Block {
    fn parse(str: &str) -> IResult<&str, Self> {
        let (str, blocks) = separated_list1(
            tag(", "),
            separated_pair(map_res(digit1, |s: &str| s.parse::<u8>()), space1, alpha1),
        )(str)?;

        let (blue, green, red) = blocks.iter().fold(
            (0, 0, 0),
            |(blue, green, red), (count, color)| match *color {
                "blue" => (blue + count, green, red),
                "green" => (blue, green + count, red),
                "red" => (blue, green, red + count),
                _ => panic!("Unknown color: {}", color.replace(' ', "X")),
            },
        );

        Ok((str, Self { blue, green, red }))
    }
}

struct Game {
    id: usize,
    blocks: Vec<Block>,
}

impl Game {
    fn new(str: &str) -> IResult<&str, Self> {
        let (str, id) = delimited(
            tag("Game "),
            map_res(digit1, |s: &str| s.parse::<usize>()),
            tag(": "),
        )(str)?;
        let (input, blocks) = separated_list1(tag("; "), Block::parse)(str)?;

        Ok((input, Self { id, blocks }))
    }
}

pub fn main() -> Result<(usize, usize), Error> {
    let file = misc::get_file();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let part_1 = file
        .content
        .trim()
        .lines()
        .filter_map(|line| {
            let (_, game) = Game::new(line).ok()?;
            if game.blocks.iter().all(|round| {
                round.red <= max_red && round.green <= max_green && round.blue <= max_blue
            }) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    let part_2 = file
        .content
        .trim()
        .lines()
        .map(|g| Game::new(g).unwrap().1)
        .map(|game| {
            game.blocks
                .iter()
                .fold((0, 0, 0), |(max_red, max_green, max_blue), round| {
                    (
                        max_red.max(round.red),
                        max_green.max(round.green),
                        max_blue.max(round.blue),
                    )
                })
        })
        .map(|(max_red, max_green, max_blue)| {
            max_red as usize * max_green as usize * max_blue as usize
        })
        .sum();

    Ok((part_1, part_2))
}
