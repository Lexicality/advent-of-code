// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

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

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
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
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
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

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "6",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
