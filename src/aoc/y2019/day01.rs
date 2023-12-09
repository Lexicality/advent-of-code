pub fn main(data: crate::DataIn) -> String {
    let mut ret: u64 = 0;
    for line in data {
        let num: u64 = line.parse().unwrap();
        let mut weight = num;
        let mut req = 0;
        loop {
            weight = (weight / 3).saturating_sub(2);
            if weight == 0 {
                break;
            }
            req += weight;
        }
        println!("Input: {num} Required: {req}");
        ret += req;
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "1",
    func: main,
    example_func: None,
});
