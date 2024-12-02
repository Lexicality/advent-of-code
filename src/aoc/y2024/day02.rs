#[derive(Debug, Clone, Copy)]
enum Direction {
    Mystery,
    Increasing,
    Decreasing,
}

fn validate_sequence(a: u32, b: u32, dir: Direction) -> Option<Direction> {
    if a.abs_diff(b) > 3 {
        print!("(ERR |{a} - {b}| > 3!) ");
        return None;
    }
    match a.cmp(&b) {
        std::cmp::Ordering::Less => {
            if matches!(dir, Direction::Increasing) {
                print!("(ERR {a} < {b}!) ");
                None
            } else {
                Some(Direction::Decreasing)
            }
        }
        std::cmp::Ordering::Equal => {
            print!("(ERR {a} == {b}!) ");
            None
        }
        std::cmp::Ordering::Greater => {
            if matches!(dir, Direction::Decreasing) {
                print!("(ERR {a} > {b}!) ");
                None
            } else {
                Some(Direction::Increasing)
            }
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    'lines: for line in data {
        print!("{line} | ");
        let mut dir = Direction::Mystery;
        let mut dampened = false;
        let mut levels = line
            .split(' ')
            .map(|c| c.parse::<u32>().unwrap())
            .peekable();

        let mut prev = levels.next().unwrap();
        // hell to work out if the first or second value should be discarded
        let second = levels.next().unwrap();
        print!("{prev} {second} ");
        let first_result = validate_sequence(prev, second, dir);
        if let Some(direction) = first_result {
            // everything's fine
            dir = direction;
            prev = second;
        } else {
            // dampening time baybe
            print!("DAMPENED (start) ");
            dampened = true;
            let third = levels.peek().unwrap();
            match (
                validate_sequence(prev, *third, dir),
                validate_sequence(second, *third, dir),
            ) {
                (None, None) => {
                    print!("DOUBLE FAIL ");
                    // they're both wrong
                    continue 'lines;
                }
                (None, Some(_)) => {
                    // need to discard the first value
                    print!("Disc 1st ");
                    prev = second;
                }
                (Some(_), _) => {
                    print!("Disc 2nd ");
                    // need to discard the second value, which we have already
                    // done by popping it
                }
            }
        }

        'levels: for num in levels {
            print!("{num} ");
            match validate_sequence(prev, num, dir) {
                Some(newdir) => dir = newdir,
                None => {
                    if !dampened {
                        dampened = true;
                        print!("DAMPENED ");
                        continue 'levels;
                    }
                    println!("UNSAFE");
                    continue 'lines;
                }
            }
            prev = num;
        }
        println!("safe!");
        ret += 1;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "2", main));
