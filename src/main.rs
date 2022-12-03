use crate::day_2::Day2;
use clap::Parser;
use day_1::Day1;

mod day_1;
mod day_2;
mod file_reader;

fn main() {
    let cli = Cli::parse();
    if cli.day == 1 {
        println!("{}", Day1::run(&cli.file_path));
    }
    println!("{}", Day2::run(&cli.file_path));
}

#[derive(Parser)]
#[command(author, version, about = "Elves AOC!", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DAY")]
    day: u32,
    #[arg(short, long, value_name = "FILE")]
    file_path: String,
}
