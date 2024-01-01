pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        let mut line = line.as_str();
        ret += line.len();
        line = line.strip_prefix('"').expect("Must have started with \"");
        line = line.strip_suffix('"').expect("Must have started with \"");
        let mut in_escape = false;
        let mut skip_n = 0;
        let mut strlen = 0;

        for c in line.chars() {
            if skip_n > 0 {
                skip_n -= 1;
                continue;
            }
            if !in_escape {
                if c == '\\' {
                    in_escape = true
                } else {
                    strlen += 1;
                }
            } else {
                in_escape = false;
                strlen += 1;
                if c == 'x' {
                    skip_n += 2;
                }
            }
        }

        ret -= strlen;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "8", main));
