pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        ret += line.escape_debug().to_string().len() + 2 - line.len();
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "8", main));
