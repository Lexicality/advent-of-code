use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Turn {
    Player1,
    Player2,
}

fn dice(value: usize) -> usize {
    (value % 100) + 1
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut p1pos: usize = data
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let mut p2pos: usize = data
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let mut p1score = 0;
    let mut p2score = 0;

    let mut ret = 0;

    for ((a, b, c), turn) in (0..)
        .tuples()
        .zip([Turn::Player1, Turn::Player2].into_iter().cycle())
    {
        let result = dice(a) + dice(b) + dice(c);
        // I could probably have done this better
        match turn {
            Turn::Player1 => {
                p1pos = (p1pos + result - 1) % 10 + 1;
                p1score += p1pos;
                if p1score >= 1000 {
                    println!("Player 1 wins! Their score is {p1score} while p2 has {p2score}. The dice was rolled {c} times!");
                    ret = p2score * (c + 1);
                    break;
                }
            }
            Turn::Player2 => {
                p2pos = (p2pos + result - 1) % 10 + 1;
                p2score += p2pos;
                if p2score >= 1000 {
                    println!("Player 2 wins! Their score is {p2score} while p1 has {p1score}. The dice was rolled {c} times!");
                    ret = p1score * (c + 1);
                    break;
                }
            }
        }
        // println!("{turn:?} rolls {a}+{b}+{c}={result} and we have {p1pos}/{p1score} and {p2pos}/{p2score}");
        // if c > 20 {
        //     break;
        // }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2021", "21", main));
