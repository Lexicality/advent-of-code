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
        let mut lasts = vec![line.last().copied().unwrap()];
        let mut iterations = 0;
        while !line.iter().all(|n| *n == 0) {
            iterations += 1;
            line = line
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
            lasts.push(line.last().copied().unwrap());
        }
        ret += lasts.into_iter().sum::<i32>();
    }

    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "9",
    func: main,
});
