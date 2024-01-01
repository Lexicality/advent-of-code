pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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

    Ok(elves[..3].iter().sum::<i32>().to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "1", main));
