fn hash(value: &str) -> usize {
    value.chars().fold(0, |mut hash, c| {
        hash += u32::from(c) as usize;
        hash *= 17;
        hash %= 256;
        hash
    })
}

fn find_target(lens: &str, target_box: &[(String, u32)]) -> Option<usize> {
    target_box
        .iter()
        .enumerate()
        .find(|(_, (name, _))| name == lens)
        .map(|(i, _)| i)
}

pub fn main(data: crate::DataIn) -> String {
    let mut boxes: Vec<Vec<(String, u32)>> = (0..=255).map(|_| Vec::new()).collect();
    for value in data.next().unwrap().split(',') {
        if value.ends_with('-') {
            let label = value.strip_suffix('-').unwrap();
            let target_box = boxes.get_mut(hash(label)).unwrap();
            if let Some(index) = find_target(label, target_box) {
                target_box.remove(index);
            }
        } else {
            let (label, focal_length) = value.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let target_box = boxes.get_mut(hash(label)).unwrap();
            if let Some(index) = find_target(label, target_box) {
                target_box[index].1 = focal_length;
            } else {
                target_box.push((label.to_owned(), focal_length));
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, data)| {
            data.into_iter()
                .enumerate()
                .map(|(j, (_, lens))| (i + 1) * (j + 1) * lens as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "15",
    func: main,
    example_func: None,
});
