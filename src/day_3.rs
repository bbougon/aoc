use crate::file_reader::file_content;
use std::fmt::{Display, Formatter};

const ALPHABET: [&u8; 52] = [
    &b"a"[0], &b"b"[0], &b"c"[0], &b"d"[0], &b"e"[0], &b"f"[0], &b"g"[0], &b"h"[0], &b"i"[0],
    &b"j"[0], &b"k"[0], &b"l"[0], &b"m"[0], &b"n"[0], &b"o"[0], &b"p"[0], &b"q"[0], &b"r"[0],
    &b"s"[0], &b"t"[0], &b"u"[0], &b"v"[0], &b"w"[0], &b"x"[0], &b"y"[0], &b"z"[0], &b"A"[0],
    &b"B"[0], &b"C"[0], &b"D"[0], &b"E"[0], &b"F"[0], &b"G"[0], &b"H"[0], &b"I"[0], &b"J"[0],
    &b"K"[0], &b"L"[0], &b"M"[0], &b"N"[0], &b"O"[0], &b"P"[0], &b"Q"[0], &b"R"[0], &b"S"[0],
    &b"T"[0], &b"U"[0], &b"V"[0], &b"W"[0], &b"X"[0], &b"Y"[0], &b"Z"[0],
];

pub struct Day3 {
    reorganisation: u32,
}

impl Day3 {
    pub(crate) fn run(file_path: &String) -> Day3 {
        Day3 {
            reorganisation: Self::reorganise(file_path).iter().sum(),
        }
    }

    fn reorganise(file_path: &String) -> Vec<u32> {
        Self::rucksacks(file_path)
            .0
            .iter()
            .map(|(first_compartment, second_compartment)| {
                ALPHABET
                    .into_iter()
                    .position(|letter| {
                        letter
                            == first_compartment
                                .clone()
                                .into_bytes()
                                .into_iter()
                                .filter(|letter| {
                                    second_compartment.clone().into_bytes().contains(letter)
                                })
                                .collect::<Vec<u8>>()
                                .first()
                                .expect("Should be found")
                    })
                    .expect("") as u32
                    + 1
            })
            .collect::<Vec<u32>>()
    }

    fn rucksacks(file_path: &String) -> Rucksacks {
        let file_content = file_content(file_path);
        Rucksacks::new(file_content.split("\n").collect())
    }
}

impl Display for Day3 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 3, Rucksack Reorganization:")?;
        writeln!(formatter, "\t- priorities: {}", self.reorganisation)?;
        Ok(())
    }
}

struct Rucksacks(Vec<(String, String)>);

impl Rucksacks {
    fn new(rucksacks: Vec<&str>) -> Rucksacks {
        let mut all_rucksacks: Vec<(String, String)> = Vec::new();
        rucksacks
            .iter()
            .filter(|str| !str.is_empty())
            .for_each(|rucksack| {
                let compartments = rucksack.split_at(rucksack.len() / 2);
                all_rucksacks.push((compartments.0.into(), compartments.1.into()));
            });
        Rucksacks(all_rucksacks)
    }
}

impl Rucksacks {}

#[cfg(test)]
mod rucksack_reorganisation {
    use crate::day_3::Day3;

    #[test]
    fn should_reorganise() {
        assert_eq!(
            Day3::run(&String::from("resources/tests/day_3_001.txt")).reorganisation,
            16
        );
        assert_eq!(
            Day3::run(&String::from("resources/tests/day_3_002.txt")).reorganisation,
            157
        );
    }
}
