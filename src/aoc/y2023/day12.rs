use itertools::Itertools;

pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    for line in data {
        let (report, groups) = line.split_once(' ').unwrap();
        let groups: Vec<u32> = groups.split(',').map(|c| c.parse()).try_collect().unwrap();
        println!("{report} {groups:?}");
        ret += 1;
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "12",
    func: main,
    example_func: None,
});
