use crate::file_reader::file_content;
use std::fmt::{Display, Formatter};

impl Day1 {
    pub(crate) fn run(file_path: &String) -> Day1 {
        let stock = Self::stock(file_path.into());
        Day1 {
            most_calories: Self::most_calories(&stock),
            top_3_most_calories_sum: Self::top_3_most_calories_sum(&stock),
        }
    }

    fn stock(file_path: &String) -> Stock {
        let file_content = file_content(file_path);
        let stocks: Vec<&str> = file_content.split("\n\n").collect();
        Stock::new(stocks)
    }

    fn top_3_most_calories_sum(stock: &Stock) -> u32 {
        let mut sums: Vec<u32> = Self::sums_of_calories(stock);
        sums.sort();
        sums.iter().rev().take(3).sum()
    }

    fn most_calories(stock: &Stock) -> u32 {
        *Self::sums_of_calories(stock)
            .iter()
            .max()
            .expect("Should have max")
    }

    fn sums_of_calories(stock: &Stock) -> Vec<u32> {
        stock
            .0
            .iter()
            .map(|calories_items| calories_items.0.iter().sum())
            .collect::<Vec<u32>>()
    }
}

#[derive(PartialEq, Debug)]
struct Stock(Vec<CaloriesItems>);

impl Stock {
    fn new(calories_stock: Vec<&str>) -> Stock {
        let mut stocks: Vec<CaloriesItems> = Vec::new();
        calories_stock.iter().for_each(|calories| {
            let values = calories
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .filter(|str| !str.is_empty())
                .map(|calories| {
                    calories
                        .parse::<u32>()
                        .expect(&format!("Could not parse '{}' as int", calories))
                })
                .collect::<Vec<u32>>();
            stocks.push(CaloriesItems(values));
        });
        Stock(stocks)
    }
}

#[derive(Debug)]
struct CaloriesItems(Vec<u32>);

impl PartialEq for CaloriesItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

#[derive(PartialEq)]
pub struct Day1 {
    most_calories: u32,
    top_3_most_calories_sum: u32,
}

impl Display for Day1 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 1, Elves stocks:")?;
        writeln!(formatter, "\t- Most calories: {}", self.most_calories)?;
        writeln!(
            formatter,
            "\t- Top three most calories: {}",
            self.top_3_most_calories_sum
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod calories {
    use crate::day_1::Day1;

    #[test]
    fn should_have_most_calories() {
        assert_eq!(
            Day1::run(&String::from("resources/tests/day_1_001.txt")).most_calories,
            6000
        );
        assert_eq!(
            Day1::run(&String::from("resources/tests/day_1_002.txt")).most_calories,
            6000
        );
        assert_eq!(
            Day1::run(&String::from("resources/tests/day_1_003.txt")).most_calories,
            19095
        );
    }

    #[test]
    fn should_have_top_three_most_calories() {
        assert_eq!(
            Day1::run(&String::from("resources/tests/day_1_004.txt")).top_3_most_calories_sum,
            19095 + 6824 + 6000
        );
    }
}
