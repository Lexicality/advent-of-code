pub fn main(data: crate::DataIn) -> String {
    let ret = 0;
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "0",
    func: main,
});
