#[path = "2022/mod.rs"]
mod twenty_twenty_two;
#[path = "2023/mod.rs"]
mod twenty_twenty_three;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Args {
    /// Year of exercise [default: 2023]
    #[arg(short = 'y', long = "year", default_value_t = 2023)]
    year: u16,

    /// Day of the month [default: 1]
    #[arg(short = 'd', long = "day", default_value_t = 1)]
    day: u8,
}

fn main() {
    let cli = Args::parse();
    match cli.year {
        2022 => match cli.day {
            1 => if let Ok((part1, part2)) = twenty_twenty_two::one::main() {
              println!("{}", part1);
              println!("{}", part2);
            },
            2 => if let Ok((part1, part2)) = twenty_twenty_two::two::main() {
              println!("{}", part1);
              println!("{}", part2);
            },
            3 => if let Ok((part1, part2)) = twenty_twenty_two::three::main() {
              println!("{}", part1);
              println!("{}", part2);
            },
            4 => if let Ok((part1, part2)) = twenty_twenty_two::four::main() {
              println!("{}", part1);
              println!("{}", part2);
            },
            5 => if let Ok((part1, part2)) = twenty_twenty_two::five::main() {
              println!("{}", part1);
              println!("{}", part2);
            },
            _ => print!("no such day!"),
        },
        2023 => match cli.day {
          1 => if let Ok((part1, part2)) = twenty_twenty_three::one::main() {
            println!("{}", part1);
            println!("{}", part2);
          },
          2 => if let Ok((part1, part2)) = twenty_twenty_three::two::main() {
            println!("{}", part1);
            println!("{}", part2); 
          },
          _ => println!("no such day!"),
        }
        _ => print!("no such year!"),
    }
}
