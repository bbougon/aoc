use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeInclusive};
use crate::file_reader::file_content;

const ALPHABET: [u8; 26] = [
    b"A"[0], b"B"[0], b"C"[0], b"D"[0], b"E"[0], b"F"[0], b"G"[0], b"H"[0], b"I"[0],
    b"J"[0], b"K"[0], b"L"[0], b"M"[0], b"N"[0], b"O"[0], b"P"[0], b"Q"[0], b"R"[0],
    b"S"[0], b"T"[0], b"U"[0], b"V"[0], b"W"[0], b"X"[0], b"Y"[0], b"Z"[0],
];

pub struct Day5 {
    top_crates: String,
}

impl Day5 {
    pub(crate) fn run(file_path: &String) -> Day5 {
        Day5 {
            top_crates: Self::rearrange(file_path),
        }
    }

    fn rearrange(file_path: &String) -> String {
        let file_content = file_content(file_path);
        let mut stacks: HashMap<u32, Vec<String>> = HashMap::new();
        let template: Vec<&str> = file_content.split("\n\n").collect();

        template
            .first()
            .expect("")
            .split('\n')
            .filter(|line| line.contains("["))
            .for_each(|line| {
                    line.as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(index, letter)| {
                        if ALPHABET.contains(letter) {
                            let option = String::from(line.chars().nth(index ).expect(""));
                            let position= (index / 4) as u32 + 1;
                            if stacks.contains_key(&position) {
                                stacks.get_mut(&position).expect("").push(option);
                            } else {
                                stacks.insert(position, vec![option]);
                            }
                        }
                    });
            });

        let moves: Vec<Move> = template
            .last()
            .expect("")
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|line| {
                let movement: Vec<&str> = line.split(' ').collect();
                Move {
                    amount: movement[1].parse::<u32>().expect(""),
                    from: movement[3].parse::<u32>().expect(""),
                    to: movement[5].parse::<u32>().expect("")
                }
            })
            .collect();

        stacks.iter_mut().for_each(|stack| stack.1.reverse());
        moves.iter().for_each(|movement| {
            let mut crates_to_move = stacks
                .get_mut(&movement.from)
                .expect("")
                .into_iter()
                .rev()
                .take(movement.amount as usize)
                .map(|el| String::from(el.clone()))
                .collect::<Vec<String>>();

            stacks
                .get_mut(&movement.to)
                .expect("").append(& mut crates_to_move);

            let range = Range { start: 0, end: movement.amount };

            range.for_each(|_| {
                stacks.get_mut(&movement.from)
                    .expect("")
                    .pop();
            });
        });

        RangeInclusive::new(1, stacks.len())
            .map(|key| {
                stacks.get_mut(&(key as u32)).expect("").last().expect("").clone()
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Display for Day5 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 5, Supply Stacks:")?;
        writeln!(formatter, "\t- crates on top: {}", self.top_crates)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Move {
    amount: u32,
    from: u32,
    to: u32
}

#[cfg(test)]
mod supply_stacks {
    use crate::day_5::Day5;

    #[test]
    fn should_rearrange_stacks() {
        assert_eq!(Day5::run(&String::from("resources/tests/day_5_001.txt")).top_crates, String::from("CMZ"));
    }
}