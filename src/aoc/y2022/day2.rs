#[derive(Debug)]
enum Result {
    Win = 6,
    Draw = 3,
    Loss = 0,
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

fn game_result(you: &Move, elf: &Move) -> Result {
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

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut total_score: u32 = 0;

    for line in data {
        let (elf, you) = line.split_once(' ').unwrap();
        let you = Move::new(you);
        let elf = Move::new(elf);
        let result = game_result(&you, &elf);

        println!("{:?} {:?} {:?}", you, elf, result);
        let move_score = you as u32;
        let result_score = result as u32;
        let round_score = move_score + result_score;
        println!("{}+{}={}", move_score, result_score, round_score);
        total_score += round_score;
        println!("{}", total_score);
    }

    return format!("{}", total_score);
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "2",
    func: main,
});
