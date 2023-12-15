pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    for value in data.next().unwrap().split(',') {
        ret += value.chars().fold(0, |mut hash, c| {
            hash += u32::from(c);
            hash *= 17;
            hash %= 256;
            hash
        });
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "15",
    func: main,
    example_func: None,
});
