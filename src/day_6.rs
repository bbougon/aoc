use crate::file_reader::file_content;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Day6 {
    first_marker_end_position: u32,
    start_up_message_end_position: u32,
}

impl Day6 {
    pub fn run(file_path: &String) -> Day6 {
        Day6 {
            first_marker_end_position: Self::find_marker_position(file_path, 4),
            start_up_message_end_position: Self::find_marker_position(file_path, 14),
        }
    }

    fn find_marker_position(file_path: &String, step: usize) -> u32 {
        let file_content = file_content(file_path);
        let content = file_content.split('\n').collect::<Vec<&str>>();
        (content
            .first()
            .expect("")
            .as_bytes()
            .windows(step)
            .map(|marker| HashSet::from_iter(marker))
            .collect::<Vec<HashSet<&u8>>>()
            .iter()
            .position(|marker| marker.len() == step)
            .expect("")
            + step) as u32
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
        writeln!(
            formatter,
            "\t- start up message position: {}",
            self.start_up_message_end_position
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

    #[test]
    fn should_give_start_up_message_end_position() {
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_001.txt")).start_up_message_end_position,
            19
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_002.txt")).start_up_message_end_position,
            23
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_003.txt")).start_up_message_end_position,
            23
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_004.txt")).start_up_message_end_position,
            29
        );
        assert_eq!(
            Day6::run(&String::from("resources/tests/day_6_005.txt")).start_up_message_end_position,
            26
        );
    }
}
