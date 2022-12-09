use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Add;
use std::rc::{Rc, Weak};
use std::str::Lines;
use harness;

#[derive(Clone, Copy, Debug)]
enum ItemType {
    File,
    Directory,
}

#[derive(Debug)]
struct Item {
    name: String,
    item_type: ItemType,
    size: RefCell<u32>,
    parent: Option<Weak<Item>>,
    children: RefCell<Vec<Rc<Item>>>,
}

impl Item {
    fn new(name: &str, item_type: ItemType, size: RefCell<u32>, parent: Option<Weak<Item>>) -> Item {
        Item { name: name.to_string(), item_type, size, parent, children: RefCell::new(vec![]) }
    }

    fn update_size(&self, size: u32) {
        let current_size = *self.size.borrow();
        *self.size.borrow_mut() = current_size.add(size);
        if let Some(p) = &self.parent {
            p.upgrade().unwrap().update_size(size);
        }
            
    }

    fn add_child(&self, item: Rc<Item>) {
        self.children.borrow_mut().push(item);
    }

    fn get_child(&self, name: String) -> Option<Rc<Item>> {
        for child in self.children.borrow().iter() {
            if child.name == name {
                return Some(child.to_owned());
            }
        }
        None
    }

    fn get_parent(&self) -> &Weak<Item> {
        match &self.parent {
            Some(p) => p,
            None => panic!("No parent found!"),
        }
    }
}

fn part1(lines: &mut Lines) -> u32 {
    let mut directories: Vec<Weak<Item>> = vec![];
    let root_item = Rc::new(Item::new("/", ItemType::Directory, RefCell::new(0), None));
    directories.push(Rc::downgrade(&root_item));
    
    let mut current_item = Rc::clone(&root_item);
    for line in lines.skip(1) {
        let splits: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') {
            // parse command
            if let Some(cmd) = line.split(' ').skip(1).next() {
                match cmd {
                    "cd" => {
                        let name = splits[2];
                        if name == ".." {
                            let weak_parent = current_item.get_parent();
                            let parent = weak_parent.upgrade().unwrap();
                            current_item = parent;
                        } else {
                            let cd = match current_item.get_child(name.to_string()) {
                                Some(cd) => cd,
                                None => panic!("Failed to change directory to {}", name),
                            };
                            current_item = cd;
                        }
                    },
                    &_ => continue,
                }
            }
        } else {
            let description = splits[0];
            let name = splits[1];

            let item_type = match description {
                "dir" => ItemType::Directory,
                &_ => ItemType::File,
            };

            let size = match item_type {
                ItemType::Directory => 0,
                ItemType::File => u32::from_str_radix(description, 10).unwrap(),
            };

            // parse files
            let new_item = Item::new(name, item_type, RefCell::new(size), Some(Rc::downgrade(&current_item)));
            let rc_new_item = Rc::new(new_item);
            
            match item_type {
                ItemType::File => current_item.update_size(size),
                ItemType::Directory => directories.push(Rc::downgrade(&rc_new_item)),
            }

            current_item.add_child(rc_new_item);
        }
    }

    let mut sum = 0;
    for weak_dir in directories {
        let dir = weak_dir.upgrade().unwrap();
        let dir_size = dir.size.borrow().to_owned();
        if dir_size < 100000 {
            sum += dir_size;
        }
    }

    sum
}

fn part2(lines: &mut Lines) -> u32 {
    let mut directories: Vec<Weak<Item>> = vec![];
    let root_item = Rc::new(Item::new("/", ItemType::Directory, RefCell::new(0), None));
    directories.push(Rc::downgrade(&root_item));
    
    let mut current_item = Rc::clone(&root_item);
    for line in lines.skip(1) {
        let splits: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') {
            // parse command
            if let Some(cmd) = line.split(' ').skip(1).next() {
                match cmd {
                    "cd" => {
                        let name = splits[2];
                        if name == ".." {
                            let weak_parent = current_item.get_parent();
                            let parent = weak_parent.upgrade().unwrap();
                            current_item = parent;
                        } else {
                            let cd = match current_item.get_child(name.to_string()) {
                                Some(cd) => cd,
                                None => panic!("Failed to change directory to {}", name),
                            };
                            current_item = cd;
                        }
                    },
                    &_ => continue,
                }
            }
        } else {
            let description = splits[0];
            let name = splits[1];

            let item_type = match description {
                "dir" => ItemType::Directory,
                &_ => ItemType::File,
            };

            let size = match item_type {
                ItemType::Directory => 0,
                ItemType::File => u32::from_str_radix(description, 10).unwrap(),
            };

            // parse files
            let new_item = Item::new(name, item_type, RefCell::new(size), Some(Rc::downgrade(&current_item)));
            let rc_new_item = Rc::new(new_item);
            
            match item_type {
                ItemType::File => current_item.update_size(size),
                ItemType::Directory => directories.push(Rc::downgrade(&rc_new_item)),
            }

            current_item.add_child(rc_new_item);
        }
    }

    let total_disk_space = 70000000;
    let required_free_disk_space = 30000000;
    let free_disk_space = total_disk_space - root_item.size.borrow().to_owned();
    let disk_space_to_free = required_free_disk_space - free_disk_space;

    let mut potential_dir = u32::MAX;
    for weak_dir in directories {
        let dir = weak_dir.upgrade().unwrap();
        let dir_size = dir.size.borrow().to_owned();
        if dir_size > disk_space_to_free && dir_size < potential_dir {
            potential_dir = dir_size;
        }
    }

    potential_dir
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
