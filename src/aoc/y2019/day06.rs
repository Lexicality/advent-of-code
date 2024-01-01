use std::collections::HashMap;

#[derive(Debug, Default)]
struct OrbitItem {
    name: String,
    parent: Option<String>,
    children: Vec<String>,
}

impl OrbitItem {
    fn new(name: &str) -> Self {
        OrbitItem {
            name: name.to_owned(),
            parent: None,
            children: Vec::with_capacity(1),
        }
    }
}

fn get_path<'a>(orbit_map: &'a HashMap<String, OrbitItem>, mut obj: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    loop {
        obj = orbit_map
            .get(obj)
            .unwrap()
            .parent
            .as_ref()
            .unwrap()
            .as_str();
        ret.push(obj);
        if obj == "COM" {
            break;
        }
    }
    ret
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut orbit_map: HashMap<String, OrbitItem> = HashMap::with_capacity({
        let hint = data.size_hint();
        hint.1.unwrap_or(hint.0) + 1
    });
    for line in data {
        let (parent_ref, object_ref) = line.split_once(')').unwrap();
        if !orbit_map.contains_key(object_ref) {
            orbit_map.insert(object_ref.to_owned(), OrbitItem::new(object_ref));
        }
        {
            let object_item = orbit_map.get_mut(object_ref).unwrap();
            object_item.parent = Some(parent_ref.to_owned());
        }

        if !orbit_map.contains_key(parent_ref) {
            orbit_map.insert(parent_ref.to_owned(), OrbitItem::new(parent_ref));
        }
        {
            let parent_item = orbit_map.get_mut(parent_ref);
            parent_item.unwrap().children.push(object_ref.to_owned());
        }
    }
    // Sanity check
    for object in orbit_map.values() {
        if object.name != "COM" && object.parent.is_none() {
            panic!("Object {} is broken!", object.name);
        }
    }
    if !orbit_map.contains_key("YOU") {
        panic!("I'm missing YOU!")
    } else if !orbit_map.contains_key("SAN") {
        panic!("I'm missing Santa!")
    }

    let mut youtree = get_path(&orbit_map, "YOU");
    let mut santree = get_path(&orbit_map, "SAN");

    // shrug
    while youtree.last() == santree.last() {
        assert_ne!(youtree.pop(), None);
        assert_ne!(santree.pop(), None);
    }

    Ok((youtree.len() + santree.len()).to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "6", main));
