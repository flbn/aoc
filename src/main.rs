#[path = "2022/mod.rs"]
mod twenty_twenty_two;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Args {
    /// Year of exercise
    #[arg(short = 'y', long = "year", default_value_t = 2022)]
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
            _ => print!("no such day!"),
        },
        _ => print!("no such year!"),
    }
}
