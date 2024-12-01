use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let (mut a, mut b): (Vec<_>, Vec<_>) = data
        .map(|line| -> (u64, u64) {
            let (a, b) = line.split_once(' ').expect("line must be splittable");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .unzip();
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "lists must have the same length!");
    Ok(a.into_iter()
        .zip_eq(b)
        .map(|(a, b)| a.abs_diff(b))
        .reduce(u64::saturating_add)
        .unwrap()
        .to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "1", main));
