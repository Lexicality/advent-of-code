use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: u32 = data
        .map(|line| {
            let mut tmp: Vec<u32> = line.split('x').map(|c| c.parse()).try_collect().unwrap();
            tmp.sort();
            let (l, w, h) = tmp.into_iter().collect_tuple().unwrap();

            let a = (l, w);
            let b = (w, h);
            let c = (l, h);
            let slack = a.min(b).min(c);

            2 * slack.0 + 2 * slack.1 + l * w * h
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "2",
    func: main,
    example_func: None,
});
