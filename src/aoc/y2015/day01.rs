use itertools::{FoldWhile, Itertools};

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    Ok(data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .enumerate()
        .fold_while((0, 0), |(_, sum), (i, v)| {
            let res = sum + v;
            if res >= 0 {
                FoldWhile::Continue((0, res))
            } else {
                FoldWhile::Done((i + 1, 0))
            }
        })
        .into_inner()
        .0
        .to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    crate::multi_line_example(data, main)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "1",
    main,
    main_example
));
