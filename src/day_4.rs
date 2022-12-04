use crate::file_reader::file_content;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

pub struct Day4 {
    number_of_pairs: u32,
}

impl Day4 {
    pub fn run(file_path: &String) -> Day4 {
        Day4 {
            number_of_pairs: Self::detect_number_of_pairs(file_path),
        }
    }

    fn detect_number_of_pairs(file_path: &String) -> u32 {
        Self::pairs(file_path, |first_elfe_range, second_elfe_range| {
            (first_elfe_range.contains(&second_elfe_range.start())
                && first_elfe_range.contains(&second_elfe_range.end()))
                || (second_elfe_range.contains(&first_elfe_range.start())
                    && second_elfe_range.contains(&first_elfe_range.end()))
        })
        .into_iter()
        .filter(|pair| pair.assignment_fully_contained)
        .count() as u32
    }

    fn pairs(
        file_path: &String,
        apply: fn(RangeInclusive<u32>, RangeInclusive<u32>) -> bool,
    ) -> Vec<Pair> {
        let file_content = file_content(file_path);
        file_content
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|str| !str.is_empty())
            .map(|pair| Pair::from(pair, apply))
            .collect()
    }
}

impl Display for Day4 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 4, Elves beach cleanup:")?;
        writeln!(
            formatter,
            "\t- number of assigned pairs fully covered: {}",
            self.number_of_pairs
        )?;
        Ok(())
    }
}

struct Pair {
    assignment_fully_contained: bool,
}

impl Pair {
    fn from(pair: &str, apply: fn(RangeInclusive<u32>, RangeInclusive<u32>) -> bool) -> Pair {
        let pair1 = pair
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|section| section.split("-").collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|section| {
                section
                    .parse::<u32>()
                    .expect(&format!("Could not parse '{}' as int", section))
            })
            .collect::<Vec<u32>>();
        Pair {
            assignment_fully_contained: apply(
                RangeInclusive::new(pair1[0], pair1[1]),
                RangeInclusive::new(pair1[2], pair1[3]),
            ),
        }
    }
}

#[cfg(test)]
mod camp_cleanup {
    use crate::day_4::Day4;

    #[test]
    fn should_have_two_pairs_that_fully_contains_each_other_assignment() {
        assert_eq!(
            Day4::run(&String::from("resources/tests/day_4_001.txt")).number_of_pairs,
            2
        );
    }
}
