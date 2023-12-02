pub fn main(data: crate::DataIn) -> String {
    let mut ret: u64 = 0;
    for line in data {
        let num: u64 = line.parse().unwrap();
        let req = (num / 3) - 2;
        println!("Input: {num} Required: {req}");
        ret += req;
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "1",
    func: main,
});
