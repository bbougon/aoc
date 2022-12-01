use std::{env, fs};

fn main() {
    println!("Elves stocks!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let stocks: Vec<&str> = file_content.split("\n").collect();
    println!(
        "Most calories: {} - Top three most calories: {}",
        most_calories(stocks.clone()),
        top_3_most_calories_sum(stocks)
    )
}

fn top_3_most_calories_sum(stocks: Vec<&str>) -> u32 {
    let mut sums: Vec<u32> = sums_of_calories(Stock::new(stocks));
    sums.sort();
    sums.iter().rev().take(3).sum()
}

fn most_calories(stocks: Vec<&str>) -> u32 {
    *sums_of_calories(Stock::new(stocks)).iter().max().unwrap()
}

fn sums_of_calories(stock: Stock) -> Vec<u32> {
    stock
        .0
        .iter()
        .map(|calories_items| calories_items.0.iter().sum())
        .collect::<Vec<u32>>()
}

#[derive(PartialEq, Debug)]
struct Stock(Vec<CaloriesItems>);

impl Stock {
    fn new(calories_stock: Vec<&str>) -> Stock {
        let mut stocks: Vec<CaloriesItems> = Vec::new();
        let mut calories_items: Vec<u32> = Vec::new();
        calories_stock.iter().for_each(|calories| {
            if *calories == "" {
                stocks.push(CaloriesItems(calories_items.clone()));
                calories_items = Vec::new();
            } else {
                calories_items.push(
                    calories
                        .parse::<u32>()
                        .expect("Could not parse string as int"),
                )
            }
        });
        stocks.push(CaloriesItems(calories_items));
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

#[cfg(test)]
mod elves_test {
    use crate::{most_calories, top_3_most_calories_sum};

    #[test]
    fn day_1() {
        assert_eq!(most_calories(vec!["1000", "2000", "3000"]), 6000);
        assert_eq!(
            most_calories(vec![
                "1000", "2000", "3000", "", "2000", "2000", "1000", "1000"
            ]),
            6000
        );
        assert_eq!(
            most_calories(vec![
                "1000", "2000", "3000", "", "2000", "2000", "1000", "1000", "", "2011", "5421",
                "5432", "6231", "", "2024", "2300", "2500"
            ]),
            19095
        );
        assert_eq!(
            top_3_most_calories_sum(vec![
                "1000", "2000", "3000", "", "2000", "2000", "1000", "1000", "", "2011", "5421",
                "5432", "6231", "", "2024", "2300", "2500", "", "1000", "", "2000"
            ]),
            19095 + 6824 + 6000
        );
    }
}
