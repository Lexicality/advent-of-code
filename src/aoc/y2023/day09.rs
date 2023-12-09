use itertools::Itertools;

#[allow(unused_variables)]
pub fn main(data: crate::DataIn) -> String {
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

    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "9",
    func: main,
    example_func: None,
});
