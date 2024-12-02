enum Direction {
    Mystery,
    Increasing,
    Decreasing,
}

fn check_line(mut results: impl Iterator<Item = u32>) -> bool {
    let mut dir = Direction::Mystery;
    let mut a = results.next().unwrap();
    for b in results {
        if a.abs_diff(b) > 3 {
            return false;
        }
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                if matches!(dir, Direction::Increasing) {
                    return false;
                }
                dir = Direction::Decreasing
            }
            std::cmp::Ordering::Equal => {
                return false;
            }
            std::cmp::Ordering::Greater => {
                if matches!(dir, Direction::Decreasing) {
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
    'lines: for line in data {
        let results: Vec<u32> = line.split(' ').map(|c| c.parse().unwrap()).collect();
        if check_line(results.iter().copied()) {
            ret += 1;
            continue;
        }
        for to_remove in 0..results.len() {
            if check_line(
                results
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != to_remove)
                    .map(|(_, v)| v)
                    .copied(),
            ) {
                ret += 1;
                continue 'lines;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "2", main));
