use crate::day_2::Day2;
use crate::day_3::Day3;
use clap::Parser;
use day_1::Day1;

mod day_1;
mod day_2;
mod day_3;
mod file_reader;

fn main() {
    let cli = Cli::parse();
    match cli.day {
        1 => println!("{}", Day1::run(&cli.file_path)),
        2 => println!("{}", Day2::run(&cli.file_path)),
        3 => println!("{}", Day3::run(&cli.file_path)),
        _ => println!("Unknown day"),
    }
}

#[derive(Parser)]
#[command(author, version, about = "Elves AOC!", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DAY")]
    day: u32,
    #[arg(short, long, value_name = "FILE")]
    file_path: String,
}
