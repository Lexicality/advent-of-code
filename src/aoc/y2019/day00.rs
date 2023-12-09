pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "0",
    func: main,
    example_func: None,
});
