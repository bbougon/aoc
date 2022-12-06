use crate::file_reader::file_content;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub struct Day6 {
    first_marker_end_position: u32,
}

impl Day6 {
    pub fn run(file_path: &String) -> Day6 {
        Day6 {
            first_marker_end_position: Self::find_marker_position(file_path),
        }
    }

    fn find_marker_position(file_path: &String) -> u32 {
        let file_content = file_content(file_path);
        let content = file_content.split('\n').collect::<Vec<&str>>();
        let datastream = content.first().expect("").as_bytes();
        let range = Range {
            start: 0,
            end: datastream.len(),
        };
        (range
            .into_iter()
            .filter(|range| range < &datastream.len().wrapping_sub(4))
            .map(|range| &datastream[range..range + 4])
            .collect::<Vec<&[u8]>>()
            .into_iter()
            .map(|marker| HashSet::from_iter(marker))
            .collect::<Vec<HashSet<&u8>>>()
            .iter()
            .position(|marker| marker.len() == 4)
            .expect("")
            + 4) as u32
    }
}

impl Display for Day6 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 6, Elves tuning trouble:")?;
        writeln!(
            formatter,
            "\t- marker position: {}",
            self.first_marker_end_position
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tuning_trouble {
    use crate::day_6::Day6;

    #[test]
    fn should_give_first_marker_end_position() {
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_001.txt")).first_marker_end_position,
            7
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_002.txt")).first_marker_end_position,
            5
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_003.txt")).first_marker_end_position,
            6
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_004.txt")).first_marker_end_position,
            10
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_005.txt")).first_marker_end_position,
            11
        );
    }
}
