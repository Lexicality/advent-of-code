// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

#[derive(Debug)]
enum Result {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Result {
    fn new(txt: &str) -> Result {
        match txt {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown result {}!", txt),
        }
    }
}

#[derive(Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn new(txt: &str) -> Move {
        match txt {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Unknown move {}!", txt),
        }
    }
}

fn game_result_part_1(you: &Move, elf: &Move) -> Result {
    // brute force
    match you {
        Move::Rock => match elf {
            Move::Rock => Result::Draw,
            Move::Paper => Result::Loss,
            Move::Scissors => Result::Win,
        },
        Move::Paper => match elf {
            Move::Rock => Result::Win,
            Move::Paper => Result::Draw,
            Move::Scissors => Result::Loss,
        },
        Move::Scissors => match elf {
            Move::Rock => Result::Loss,
            Move::Paper => Result::Win,
            Move::Scissors => Result::Draw,
        },
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut total_score: u32 = 0;

    for line in data {
        let (elf, you) = line.split_once(' ').unwrap();
        let you = Move::new(you);
        let elf = Move::new(elf);
        let result = game_result_part_1(&you, &elf);

        println!("{:?} {:?} {:?}", you, elf, result);
        let move_score = you as u32;
        let result_score = result as u32;
        let round_score = move_score + result_score;
        println!("{}+{}={}", move_score, result_score, round_score);
        total_score += round_score;
        println!("{}", total_score);
    }

    Ok(total_score.to_string())
}

fn game_result_part_2(you: &Result, elf: &Move) -> Move {
    // brute force
    match elf {
        Move::Rock => match you {
            Result::Draw => Move::Rock,
            Result::Loss => Move::Scissors,
            Result::Win => Move::Paper,
        },
        Move::Paper => match you {
            Result::Win => Move::Scissors,
            Result::Draw => Move::Paper,
            Result::Loss => Move::Rock,
        },
        Move::Scissors => match you {
            Result::Loss => Move::Paper,
            Result::Win => Move::Rock,
            Result::Draw => Move::Scissors,
        },
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut total_score: u32 = 0;

    for line in data {
        let (elf, result) = line.split_once(' ').unwrap();
        let result = Result::new(result);
        let elf = Move::new(elf);
        let you = game_result_part_2(&result, &elf);

        println!("{:?} {:?} {:?}", result, elf, you);
        let move_score = result as u32;
        let result_score = you as u32;
        let round_score = move_score + result_score;
        println!("{}+{}={}", move_score, result_score, round_score);
        total_score += round_score;
        println!("{}", total_score);
    }

    Ok(total_score.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
