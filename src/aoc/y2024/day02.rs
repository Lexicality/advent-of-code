use itertools::Itertools;

enum Direction {
    Mystery,
    Increasing,
    Decreasing,
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    'lines: for line in data {
        print!("{line} ");
        let mut dir = Direction::Mystery;
        for (a, b) in line
            .split(' ')
            .map(|c| c.parse::<u32>().unwrap())
            .tuple_windows()
        {
            if a.abs_diff(b) > 3 {
                println!("UNSAFE (too big a change)");
                continue 'lines;
            }
            match a.cmp(&b) {
                std::cmp::Ordering::Less => {
                    if matches!(dir, Direction::Increasing) {
                        println!("UNSAFE (dir change)");
                        continue 'lines;
                    }
                    dir = Direction::Decreasing
                }
                std::cmp::Ordering::Equal => {
                    println!("UNSAFE (equality)");
                    continue 'lines;
                }
                std::cmp::Ordering::Greater => {
                    if matches!(dir, Direction::Decreasing) {
                        println!("UNSAFE (dir change)");
                        continue 'lines;
                    }
                    dir = Direction::Increasing
                }
            }
        }
        println!("safe!");
        ret += 1;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "2", main));
