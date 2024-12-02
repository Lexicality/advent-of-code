enum Direction {
    Mystery,
    Increasing,
    Decreasing,
}

fn check_line(line: &str, mut dampening: bool) -> bool {
    print!("{line} || ");
    let mut dir = Direction::Mystery;
    let mut results = line.split(' ').map(|c| c.parse::<u32>().unwrap());

    let mut a = results.next().unwrap();
    print!("{a} ");

    for b in results {
        print!("{b} ");
        if a.abs_diff(b) > 3 {
            print!("(ERR |{a} - {b}| == {} > 3!) ", a.abs_diff(b));
            if !dampening {
                print!("damp! ");
                dampening = true;
                continue;
            }
            return false;
        }
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                if matches!(dir, Direction::Increasing) {
                    print!("(ERR {a} < {b}!) ");
                    if !dampening {
                        print!("damp! ");
                        dampening = true;
                        continue;
                    }
                    return false;
                }
                dir = Direction::Decreasing
            }
            std::cmp::Ordering::Equal => {
                print!("(ERR {a} == {b}!) ");
                if !dampening {
                    print!("damp! ");
                    dampening = true;
                    continue;
                }
                return false;
            }
            std::cmp::Ordering::Greater => {
                if matches!(dir, Direction::Decreasing) {
                    print!("(ERR {a} > {b}!) ");
                    if !dampening {
                        print!("damp! ");
                        dampening = true;
                        continue;
                    }
                    return false;
                }
                dir = Direction::Increasing
            }
        }
        a = b;
    }
    true
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        if check_line(&line, false) {
            ret += 1;
            println!("safe!");
            continue;
        }
        println!("RETRY");
        let (_, line) = line.split_once(' ').unwrap();
        if check_line(line, true) {
            println!("safe!");
            ret += 1;
            continue;
        }
        println!("UNSAFE");
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "2", main));
