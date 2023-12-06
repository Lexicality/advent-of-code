#[allow(unused_variables, unused_mut)]
pub fn main(data: crate::DataIn) -> String {
    let times = data.next().unwrap();
    let mut times = times.split_whitespace();
    let distances = data.next().unwrap();
    let mut distances = distances.split_whitespace();
    assert_eq!(times.next(), Some("Time:"));
    assert_eq!(distances.next(), Some("Distance:"));
    let mut ret = 0;
    for (time, record) in times.zip(distances) {
        let race_time: u32 = time.parse().unwrap();
        let record: u32 = record.parse().unwrap();
        let mut winnables = 0;
        let mut hump = false;
        for hold_time in 0..race_time {
            let time = race_time - hold_time;
            let distance = hold_time * time;
            if distance < record && hump {
                break;
            } else if distance > record {
                hump = true;
                winnables += 1;
            }
        }
        if ret == 0 {
            ret = winnables;
        } else {
            ret *= winnables;
        }
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "6",
    func: main,
});
