pub fn main(data: crate::DataIn) -> String {
    let ret = 0;
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "1",
    func: main,
});
