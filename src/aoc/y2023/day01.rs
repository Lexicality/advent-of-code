pub fn main(data: crate::DataIn) -> String {
    let mut total = 0;
    for line in data {
        let first = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("at least one number");
        let last = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .expect("at least one number");
        let value: u64 = format!("{first}{last}").parse().unwrap();
        println!("{line} {value}");
        total += value
    }
    format!("{total}")
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "1",
    func: main,
});
