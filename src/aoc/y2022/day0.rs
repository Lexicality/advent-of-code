pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    return data.next().unwrap();
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "0",
    func: main,
});
