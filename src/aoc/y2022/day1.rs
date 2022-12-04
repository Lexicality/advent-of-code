pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut elves = vec![];
    let mut running_elf = 0;
    for line in data {
        let line = line.trim();
        if line.is_empty() {
            elves.push(running_elf);
            running_elf = 0;
            continue;
        }
        running_elf += line.parse::<i32>().unwrap();
    }
    if running_elf > 0 {
        elves.push(running_elf);
    }

    elves.sort_unstable();
    elves.reverse();

    return format!("{}", elves[..3].iter().sum::<i32>());
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "1",
    func: main,
});
