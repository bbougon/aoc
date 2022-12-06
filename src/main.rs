use crate::day_2::Day2;
use crate::day_3::Day3;
use crate::day_4::Day4;
use crate::day_5::Day5;
use crate::day_6::Day6;
use clap::Parser;
use day_1::Day1;
use std::error::Error;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod file_reader;

fn main() {
    let cli = Cli::parse();
    match cli.day.as_str() {
        "1" => println!("{}", Day1::run(&cli.file_path)),
        "2" => println!("{}", Day2::run(&cli.file_path)),
        "3" => println!("{}", Day3::run(&cli.file_path)),
        "4" => println!("{}", Day4::run(&cli.file_path)),
        "5" => println!("{}", Day5::run(&cli.file_path)),
        "6" => println!("{}", Day6::run(&cli.file_path)),
        "all" => {
            let files_path: Vec<(String, String)> = cli.all_files.expect("");
            files_path
                .iter()
                .for_each(|(day, path)| match day.as_str() {
                    "day1" => println!("{}", Day1::run(path)),
                    "day2" => println!("{}", Day2::run(path)),
                    "day3" => println!("{}", Day3::run(path)),
                    "day4" => println!("{}", Day4::run(path)),
                    "day5" => println!("{}", Day5::run(path)),
                    "day6" => println!("{}", Day6::run(path)),
                    _ => println!("Unknown day"),
                });
        }
        _ => println!("Unknown day"),
    }
}

#[derive(Parser)]
#[command(author, version, about = "Elves AOC!", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DAY")]
    day: String,
    #[arg(short, long, value_name = "FILE")]
    file_path: String,
    #[arg(short, long, value_parser = parse_key_val::<String, String>, value_name = "FILE")]
    all_files: Option<Vec<(String, String)>>,
}
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
