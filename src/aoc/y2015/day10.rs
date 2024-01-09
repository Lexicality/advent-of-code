fn look_and_say(line: String) -> String {
    let mut ret = String::new();

    let mut cchar = line.chars().next().unwrap();
    let mut ccount = 0;
    for num in line.chars() {
        if num == cchar {
            ccount += 1;
        } else {
            ret += &ccount.to_string();
            ret.push(cchar);
            cchar = num;
            ccount = 1;
        }
    }
    ret += &ccount.to_string();
    ret.push(cchar);
    ret
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut line = data.next().unwrap();
    println!("{line}");
    for _ in 0..40 {
        line = look_and_say(line);
        println!("{line}");
    }
    Ok(line.len().to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        println!("Input: {line}");
        let ret = look_and_say(line);
        println!("Output: {ret}");
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "10",
    main,
    main_example
));
