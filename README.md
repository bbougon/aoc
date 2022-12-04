# AOC 2022

## Commands
- `cargo test` run tests
- `cargo build` build the solution, then you can use the `elves` executable:
  - ` ./target/debug/elves --help`:
    ```bash
    Elves AOC!
    
    Usage: elves [OPTIONS] --day <DAY> --file-path <FILE>
    
    Options:
    -d, --day <DAY>         
    -f, --file-path <FILE>  
    -a, --all-files <FILE>  
    -h, --help              Print help information
    -V, --version           Print version information
    ```
- `cargo run -- -d 2 -f resources/main/day_2.txt` will run day 2 `-d 2` of AOC 2022 with input file `-f resources/main/day_2.txt`
- `cargo run -- -d all -a day1=resources/main/day_1.txt -a day2=resources/main/day_2.txt -a day3=resources/main/day_3.txt -f None` will run all defined days:
  `-a day1=resources/main/day_1.txt` for day 1 ...