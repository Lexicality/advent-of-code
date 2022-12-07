use std::collections::HashSet;

type Pocket = HashSet<char>;

fn get_priority(item: char) -> u32 {
    if !item.is_ascii_alphabetic() {
        panic!("Unknowable item: {}", item);
    }

    if item.is_ascii_uppercase() {
        return item as u32 - ('A' as u32 - 26 - 1);
    }
    return item as u32 - ('a' as u32 - 1);
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut total_prio = 0;
    for line in data {
        let len = line.len();
        if len % 2 != 0 {
            panic!("Line {} is uneven!", line);
        }
        let mid = len / 2;
        let (contents1, contents2) = line.split_at(mid);
        let pocket1: Pocket = contents1.chars().collect();
        let pocket2: Pocket = contents2.chars().collect();
        let intersection: Vec<_> = pocket1.intersection(&pocket2).collect();
        if intersection.len() != 1 {
            panic!("Got multiple intersections: {:?}", intersection)
        }
        let common_item = *intersection[0];
        let prio = get_priority(common_item);
        println!("{}: {}/{}", line, common_item, prio);
        total_prio += prio;
    }
    format!("{}", total_prio)
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "3",
    func: main,
});
