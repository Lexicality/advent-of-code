// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

type INoder = RefCell<INode>;

enum INode {
    File {
        name: String,
        size: u64,
        parent: Weak<INoder>,
    },
    Folder {
        name: String,
        this: Weak<INoder>,
        children: Vec<Rc<INoder>>,
        parent: Weak<INoder>,
    },
}

impl INode {
    fn name(&self) -> &str {
        match self {
            Self::File { name, .. } => name,
            Self::Folder { name, .. } => name,
        }
    }
    fn parent(&self) -> &Weak<INoder> {
        match self {
            Self::File { parent, .. } => parent,
            Self::Folder { parent, .. } => parent,
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::File { size, .. } => *size,
            Self::Folder { children, .. } => children
                .iter()
                .map(|c| Borrow::<INoder>::borrow(c).borrow().size())
                .sum(),
        }
    }

    fn new_folder(name: String, parent: Weak<INoder>) -> Rc<INoder> {
        Rc::new_cyclic(move |this| {
            RefCell::new(Self::Folder {
                name,
                this: this.clone(),
                parent,
                children: Vec::new(),
            })
        })
    }

    fn add_file(&mut self, name: String, size: u64) {
        let (this, children) = match self {
            Self::File { .. } => panic!("I'm a file!"),
            Self::Folder { this, children, .. } => (this, children),
        };
        children.push(Rc::new(RefCell::new(Self::File {
            name,
            size,
            parent: this.clone(),
        })));
    }

    fn add_folder(&mut self, name: String) -> Rc<INoder> {
        let (this, children) = match self {
            Self::File { .. } => panic!("I'm a file!"),
            Self::Folder { this, children, .. } => (this, children),
        };
        let folder = Self::new_folder(name, this.clone());
        children.push(folder.clone());
        folder
    }

    fn get_child(&self, name: &str) -> Rc<INoder> {
        let children = match self {
            Self::File { .. } => panic!("I'm a file!"),
            Self::Folder { children, .. } => children,
        };
        children
            .iter()
            // .filter(|c| Borrow::<INoder>::borrow(c).borrow().name() == name);
            .find(|c| Borrow::<INoder>::borrow(*c).borrow().name() == name)
            .unwrap()
            .clone()
    }

    fn depth(&self) -> usize {
        let mut parent = self.parent().clone();
        let mut depth = 0;
        while let Some(gp) = parent.upgrade() {
            depth += 1;
            parent = Borrow::<INoder>::borrow(&gp).borrow().parent().clone();
        }
        depth
    }
}

impl Display for INode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:width$}{} {} ({})",
            "",
            match self {
                INode::File { .. } => "- ",
                INode::Folder { .. } => "/ ",
            },
            self.name(),
            self.size(),
            width = self.depth()
        )?;
        if let INode::Folder { children, .. } = self {
            for child in children {
                write!(f, "{}", Borrow::<INoder>::borrow(child).borrow())?;
            }
        }
        Ok(())
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let root = INode::new_folder("/".to_string(), Weak::new());
    // let mut folders: HashMap<String, Rc<INoder>> = HashMap::new();
    let mut folders: Vec<Rc<INoder>> = Vec::new();
    folders.push(root.clone());
    let mut active_folder = root.clone();

    for line in data.skip(1) {
        // println!(
        //     "Current:\n{}line: '{}'",
        //     Borrow::<INoder>::borrow(&root).borrow(),
        //     line
        // );
        println!("{}", line);
        if line.starts_with('$') {
            // it's a command
            if line == "$ ls" {
                continue;
            } else if line.starts_with("$ cd") {
                let dir = &line[5..];
                println!("Changing to '{}'", dir);
                active_folder = if dir == ".." {
                    Borrow::<INoder>::borrow(&active_folder)
                        .borrow()
                        .parent()
                        .upgrade()
                        .unwrap()
                } else {
                    Borrow::<INoder>::borrow(&active_folder)
                        .borrow()
                        .get_child(dir)
                };
            }
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                // it's a dir
                println!(
                    "Adding directory '{}' to {}",
                    name,
                    active_folder.borrow_mut().name()
                );
                let f = active_folder.borrow_mut().add_folder(name.to_string());
                folders.push(f);
            } else {
                // it's a file!
                println!(
                    "Adding file '{}' ({}) to {}",
                    name,
                    size,
                    active_folder.borrow_mut().name()
                );
                active_folder
                    .borrow_mut()
                    .add_file(name.to_string(), size.parse().unwrap())
            }
        }
    }
    println!("{}", Borrow::<INoder>::borrow(&root).borrow());

    let total_space = 70_000_000;
    let total_needed = 30_000_000;
    let left = total_space - root.borrow_mut().size();
    let needed = total_needed - left;

    let size: u64 = folders
        .iter()
        .map(|d| {
            let dir = Borrow::<INoder>::borrow(d).borrow();
            let size = dir.size();
            println!("Folder {} is {}", dir.name(), size);
            size
        })
        .filter(|s| s >= &needed)
        .inspect(|&s| {
            println!("{}", s);
        })
        .min()
        .unwrap();

    Ok(size.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "7",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
