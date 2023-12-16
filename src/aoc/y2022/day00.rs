pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "0",
    func: main,
    example_func: None,
});
