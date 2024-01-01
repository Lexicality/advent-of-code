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
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unknown move {}!", txt),
        }
    }
}

fn game_result(you: &Result, elf: &Move) -> Move {
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

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut total_score: u32 = 0;

    for line in data {
        let (elf, result) = line.split_once(' ').unwrap();
        let result = Result::new(result);
        let elf = Move::new(elf);
        let you = game_result(&result, &elf);

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

inventory::submit!(crate::AoCDay::mew("2022", "2", main));
