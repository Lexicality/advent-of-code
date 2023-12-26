pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    Ok(data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum::<i32>()
        .to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let res = main(&mut vec![line].into_iter())?;
        println!("=== {res} ===");
    }
    Ok("".to_owned())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "1",
    func: main,
    example_func: Some(main_example),
});
