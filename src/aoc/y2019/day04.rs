use std::cmp::Ordering;

fn check_num(num: u32) -> bool {
    let num = num.to_string();
    if num.len() != 6 {
        return false;
    }
    let mut digits = num.chars().map(|c| c.to_digit(10).unwrap());
    let mut prev = digits.next().unwrap();
    let mut doubles = false;
    for digit in digits {
        match digit.cmp(&prev) {
            Ordering::Less => return false,
            Ordering::Equal => doubles = true,
            Ordering::Greater => (),
        };
        prev = digit;
    }
    doubles
}

pub fn main(data: crate::DataIn) -> String {
    let line = data.next().unwrap();
    let (start, end) = line.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

    let mut ret = 0;
    for num in start..end {
        if check_num(num) {
            ret += 1;
        }
    }

    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "4",
    func: main,
});

#[cfg(test)]
mod test {
    use super::check_num;

    #[test]
    fn adjacent() {
        assert!(check_num(122345));
    }

    #[test]
    fn increasing() {
        assert!(check_num(111123));
        // assert!(check_num(135679));
    }

    #[test]
    fn oneoneone() {
        assert!(check_num(111111));
    }

    #[test]
    fn zero() {
        assert!(!check_num(223450));
    }

    #[test]
    fn no_doubs() {
        assert!(!check_num(123789));
    }

    #[test]
    fn len() {
        assert!(!check_num(11378));
    }
}
