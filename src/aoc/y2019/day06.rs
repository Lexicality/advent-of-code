use std::collections::{HashMap, VecDeque};

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

pub fn main(data: crate::DataIn) -> String {
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

    let mut ret = 0;
    let mut to_manage: VecDeque<_> = VecDeque::with_capacity(orbit_map.len() / 2);
    let mut depths: HashMap<&str, u32> = HashMap::with_capacity(orbit_map.capacity());
    depths.insert("COM", 0);
    to_manage.push_back("COM");
    while let Some(object_ref) = to_manage.pop_front() {
        if object_ref != "COM" {
            let object = orbit_map.get(object_ref).unwrap();
            let parent = object.parent.as_ref().unwrap();
            let depth = depths.get(parent.as_str()).unwrap() + 1;
            depths.insert(object_ref, depth);
            ret += depth;
        }
        to_manage.extend(
            orbit_map
                .get(object_ref)
                .unwrap()
                .children
                .iter()
                .map(|s| s.as_str()),
        );
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "6",
    func: main,
});
