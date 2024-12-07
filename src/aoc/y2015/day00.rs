pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret = data.count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "0", main));
