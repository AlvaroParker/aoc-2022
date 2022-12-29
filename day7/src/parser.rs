use std::fs;

use crate::tree::{self, ArenaTree};
// AsRef<OsStr>
enum Action {
    Dir(String),
    File(String, u32),
    Cd(String),
    None,
}
fn open() -> String {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(e) => {
            panic!("Error while reading file: {}", e)
        }
    };
    content
}

pub fn parse() -> ArenaTree {
    let content = open();
    let mut wd = 0;
    let mut arena = tree::ArenaTree::new();
    arena.new_root(tree::Directory::new("root".to_string(), None));

    for line in content.lines().skip(1) {
        let action = set_action(line);
        match action {
            Action::Dir(s) => {
                arena.new_dir_with_parent(tree::Directory::new(s, Some(wd)));
            }
            Action::File(name, size) => {
                arena.add_file(wd, tree::File::new(name, size));
            }
            Action::Cd(s) => {
                wd = arena.cd(s, wd);
            }
            Action::None => {}
        }
    }
    arena
}

fn set_action(s: &str) -> Action {
    if s.starts_with("$") {
        if s.starts_with("$ cd") {
            let v = s.split_whitespace().collect::<Vec<_>>();
            return Action::Cd(v.get(2).unwrap().to_string());
        } else {
            return Action::None;
        }
    } else {
        let v = s.split_whitespace().collect::<Vec<_>>();
        if s.starts_with("dir") {
            return Action::Dir(v.get(1).unwrap().to_string());
        } else {
            let size = v.get(0).unwrap().parse::<u32>().unwrap();
            return Action::File(v.get(1).unwrap().to_string(), size);
        }
    }
}
