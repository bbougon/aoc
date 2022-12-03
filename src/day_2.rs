use crate::file_reader::file_content;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Shape {
    name: Box<str>,
    points: u32,
}

impl TryFrom<&str> for Shape {
    type Error = ();

    fn try_from(shape: &str) -> Result<Self, Self::Error> {
        match shape {
            "Y" => Ok(Paper::default().0),
            "B" => Ok(Paper::default().0),
            "X" => Ok(Rock::default().0),
            "A" => Ok(Rock::default().0),
            "Z" => Ok(Scissors::default().0),
            "C" => Ok(Scissors::default().0),
            &_ => Err(()),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Could not find any shape")
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name == "Rock".into() && other.name == "Scissors".into() {
            return Some(Greater);
        }
        if self.name == "Scissors".into() && other.name == "Paper".into() {
            return Some(Greater);
        }
        if self.name == "Paper".into() && other.name == "Rock".into() {
            return Some(Greater);
        }
        return Some(Less);
    }
}

impl PartialEq<Self> for Shape {
    fn eq(&self, other: &Self) -> bool {
        other.name == self.name
    }
}

struct Scissors(Shape);

impl Default for Scissors {
    fn default() -> Self {
        Scissors {
            0: Shape {
                name: "Scissors".into(),
                points: 3,
            },
        }
    }
}

struct Paper(Shape);

impl Default for Paper {
    fn default() -> Self {
        Paper {
            0: Shape {
                name: "Paper".into(),
                points: 2,
            },
        }
    }
}

struct Rock(Shape);

impl Default for Rock {
    fn default() -> Self {
        Rock {
            0: Shape {
                name: "Rock".into(),
                points: 1,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Round<'a>((&'a Shape, &'a Shape));

impl<'a> Round<'a> {
    fn score(&self) -> Result<u32, ()> {
        match self.0 {
            (opponent, me) => Ok(self.play(opponent, me)),
        }
    }

    fn play(&self, opponent: &Shape, me: &Shape) -> u32 {
        if opponent > me {
            return me.points;
        }
        if me > opponent {
            return me.points + 6;
        }
        me.points + 3
    }
}

#[derive(PartialEq, Debug)]
struct Rounds(Vec<(String, String)>);

impl Rounds {
    fn new(rounds: Vec<&str>) -> Rounds {
        let mut all_rounds: Vec<(String, String)> = Vec::new();
        rounds
            .iter()
            .filter(|str| !str.is_empty())
            .for_each(|round| {
                let players = round.split(" ").collect::<Vec<&str>>();
                let opponent = players[0];
                let me = players[1];
                all_rounds.push((opponent.into(), me.into()));
            });
        Rounds(all_rounds)
    }
}

pub struct Day2 {
    crypted_score: u32,
}

impl Day2 {
    pub fn run(file_path: &String) -> Day2 {
        Day2 {
            crypted_score: Self::crypted_cheat_sheet(file_path).iter().sum(),
        }
    }

    fn crypted_cheat_sheet(file_path: &String) -> Vec<u32> {
        let rounds = Self::rounds(file_path)
            .0
            .into_iter()
            .map(|(opponent, me)| {
                Round((
                    &Shape::try_from(opponent.as_str()).expect(&format!("opponent: {}", opponent)),
                    &Shape::try_from(me.as_str()).expect(&format!("me: {}", me)),
                ))
                .score()
                .unwrap()
            })
            .collect::<Vec<u32>>();
        rounds
    }

    fn rounds(file_path: &String) -> Rounds {
        let file_content = file_content(file_path);
        Rounds::new(file_content.split("\n").collect())
    }
}

impl Display for Day2 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "Day 2, Elves games:")?;
        writeln!(formatter, "\t- High score: {}", self.crypted_score)?;
        Ok(())
    }
}

#[cfg(test)]
mod rock_paper_scissors {
    use crate::day_2::Day2;

    #[test]
    fn should_give_winner_score() {
        assert_eq!(
            Day2::run(&String::from("resources/tests/day_2_001.txt")).crypted_score,
            8
        );
        assert_eq!(
            Day2::run(&String::from("resources/tests/day_2_002.txt")).crypted_score,
            15
        );
        assert_eq!(
            Day2::run(&String::from("resources/tests/day_2_003.txt")).crypted_score,
            15
        );
        assert_eq!(
            Day2::run(&String::from("resources/tests/day_2_004.txt")).crypted_score,
            45
        );
    }
}
