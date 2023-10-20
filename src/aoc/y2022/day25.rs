fn desnafu(input: String) -> i32 {
    let mut ret = 0;
    let mut amt = 1;

    for char in input.chars().rev() {
        let digit = match char {
            '-' => -1,
            '=' => -2,
            i => i.to_digit(10).unwrap() as i32,
        };
        ret += digit * amt;
        amt *= 5;
    }

    ret
}

fn ensnafu(num: i64) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut amt = 5;
    let mut count = 2;
    while num / amt != 0 {
        amt *= 5;
        count += 1;
    }
    println!("{}, {} / {}", num, amt, count);

    let mut ret = String::with_capacity(count);
    while count > 0 {
        // AAAAAA
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut ret: i64 = 0;
    for line in data {
        print!("{:10} ", line);
        let res = desnafu(line);
        println!("{:10}", res);
        ret += res as i64;
    }
    println!("{}", ret);
    return ensnafu(ret);
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "25",
    func: main,
});
