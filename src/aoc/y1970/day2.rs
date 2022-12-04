pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    println!("Hi it's day 2");
    for line in data {
        println!("Got line {}", line);
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "1970",
    day: "2",
    func: main,
});
