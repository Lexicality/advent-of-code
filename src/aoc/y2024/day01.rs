use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let (a, b): (Vec<_>, Vec<_>) = data
        .map(|line| -> (u64, u64) {
            let (a, b) = line.split_once(' ').expect("line must be splittable");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .unzip();

    let counts = b.into_iter().counts();
    Ok(a.into_iter()
        .map(|a| a * counts.get(&a).map(|b| *b as u64).unwrap_or_default())
        .reduce(u64::saturating_add)
        .unwrap()
        .to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "1", main));
