use itertools::Itertools;

#[allow(unused_variables)]
pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let lines = data.map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect_vec()
    });

    let mut ret = 0;
    for mut line in lines {
        let mut firsts = vec![line.first().copied().unwrap()];
        while !line.iter().all(|n| *n == 0) {
            line = line
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
            firsts.push(line.first().copied().unwrap());
        }
        ret += firsts
            .into_iter()
            .rev()
            .reduce(|acc, value| value - acc)
            .unwrap();
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2023", "9", main));
