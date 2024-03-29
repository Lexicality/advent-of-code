use md5::{Digest, Md5};

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let input = data.next().unwrap();
    let mut hasher = Md5::new();
    hasher.update(input);
    for i in (0..).map(|v| v.to_string()) {
        let mut hasher = hasher.clone();
        hasher.update(&i);
        let result = hasher.finalize();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return Ok(i);
        }
    }
    unreachable!()
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    crate::multi_line_example(data, main)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "4",
    main,
    main_example
));
