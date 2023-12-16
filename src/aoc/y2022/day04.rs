struct Range {
    start: u32,
    end: u32,
}
impl Range {
    fn new(input: &str) -> Range {
        let (start, end) = input.split_once('-').unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn intersects(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut overlap = 0;
    for pair in data {
        let (elf1, elf2) = pair.split_once(',').unwrap();
        let elf1 = Range::new(elf1);
        let elf2 = Range::new(elf2);
        if elf1.intersects(&elf2) || elf2.intersects(&elf1) {
            overlap += 1;
        }
    }
    Ok(overlap.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "4",
    func: main,
    example_func: None,
});
