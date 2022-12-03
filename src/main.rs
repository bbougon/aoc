use clap::Parser;
use day_1::Day1;

mod day_1;

fn main() {
    let cli = Cli::parse();
    println!("{}", Day1::run(&cli.file_path));
}

#[derive(Parser)]
#[command(author, version, about = "Elves AOC!", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file_path: String,
}
