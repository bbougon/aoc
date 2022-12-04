use crate::file_reader::file_content;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

pub struct Day4 {
    number_of_pairs: u32,
    number_of_pairs_partly_overlapping: u32,
}

impl Day4 {
    pub fn run(file_path: &String) -> Day4 {
        Day4 {
            number_of_pairs: Self::detect_number_of_pairs(
                file_path,
                |first_elfe_range, second_elfe_range| {
                    (first_elfe_range.contains(second_elfe_range.start())
                        && first_elfe_range.contains(second_elfe_range.end()))
                        || (second_elfe_range.contains(first_elfe_range.start())
                            && second_elfe_range.contains(first_elfe_range.end()))
                },
            ),
            number_of_pairs_partly_overlapping: Self::detect_number_of_pairs(
                file_path,
                |first_elfe_range, second_elfe_range| {
                    first_elfe_range.contains(second_elfe_range.start())
                        || first_elfe_range.contains(second_elfe_range.end())
                        || second_elfe_range.contains(first_elfe_range.start())
                        || second_elfe_range.contains(first_elfe_range.end())
                },
            ),
        }
    }

    fn detect_number_of_pairs(
        file_path: &String,
        is_overlapping: fn(RangeInclusive<u32>, RangeInclusive<u32>) -> bool,
    ) -> u32 {
        file_content(file_path)
            .split('\n')
            .map(|line| Pair::from(line, is_overlapping))
            .filter(|pair| pair.assignment_fully_contained)
            .count() as u32
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
        writeln!(
            formatter,
            "\t- number of assigned pairs partly covered: {}",
            self.number_of_pairs_partly_overlapping
        )?;
        Ok(())
    }
}

struct Pair {
    assignment_fully_contained: bool,
}

impl Pair {
    fn from(
        line: &str,
        is_overlapping: fn(RangeInclusive<u32>, RangeInclusive<u32>) -> bool,
    ) -> Pair {
        let pair = line
            .split(',')
            .flat_map(|section| section.split('-').collect::<Vec<&str>>())
            .map(|section| {
                section
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("Could not parse '{}' as int", section))
            })
            .collect::<Vec<u32>>();
        Pair {
            assignment_fully_contained: is_overlapping(
                RangeInclusive::new(pair[0], pair[1]),
                RangeInclusive::new(pair[2], pair[3]),
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
        assert_eq!(
            Day4::run(&String::from("resources/tests/day_4_001.txt"))
                .number_of_pairs_partly_overlapping,
            4
        );
    }
}
