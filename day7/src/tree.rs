use std::{
    ops::{Deref, DerefMut},
    usize,
};
#[derive(Debug)]
pub struct ArenaTree {
    arena: Vec<Directory>,
    size: usize,
}
#[derive(Debug)]
pub struct Directory {
    name: String,
    files: Files,
    parent: Option<usize>,
    children: Vec<usize>,
}
#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}
#[derive(Debug)]
struct Files {
    v: Vec<File>,
}

impl Deref for Files {
    type Target = Vec<File>;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
impl File {
    pub fn new(name: String, size: u32) -> File {
        File { name, size }
    }
}
impl Files {
    pub fn new() -> Files {
        Files { v: Vec::new() }
    }
    pub fn size(&self) -> u32 {
        let mut count = 0;
        self.iter().for_each(|f| count += f.size);
        count
    }
}
impl Directory {
    pub fn new(name: String, parent: Option<usize>) -> Directory {
        Directory {
            name,
            files: Files::new(),
            parent,
            children: Vec::new(),
        }
    }
    pub fn add_children(&mut self, child: usize) {
        self.children.push(child);
    }
    pub fn add_childrens<T>(&mut self, childs: T)
    where
        T: Iterator<Item = usize>,
    {
        for child in childs {
            self.children.push(child);
        }
    }
}
impl ArenaTree {
    pub fn new_root(&mut self, dir: Directory) {
        self.arena.push(dir);
    }
    pub fn new() -> ArenaTree {
        ArenaTree {
            arena: Vec::new(),
            size: 0,
        }
    }
    pub fn new_dir_with_parent(&mut self, dir: Directory) {
        self.size += 1;
        self.arena
            .get_mut(dir.parent.unwrap())
            .unwrap()
            .children
            .push(self.size);
        self.arena.push(dir);
    }
    pub fn size(&self, wd: usize) -> u32 {
        let file_size = self.arena[wd].files.size();
        let mut dir_size = 0;
        for child in &self.arena[wd].children {
            dir_size += self.size(*child);
        }
        file_size + dir_size
    }
    pub fn add_file(&mut self, wd: usize, file: File) {
        self.arena[wd].files.push(file);
    }
    pub fn cd(&self, name: String, wd: usize) -> usize {
        if name == "..".to_string() {
            return self.arena[wd].parent.unwrap();
        }
        for child in &self.arena[wd].children {
            if self.arena[*child].name == name {
                return child.clone();
            }
        }
        wd
    }
    pub fn find_dir(&self, wd: usize) -> &Directory {
        &self.arena[wd]
    }
    pub fn under100k(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for (i, _) in self.arena.iter().enumerate() {
            let size = self.size(i);
            if size <= 100000 {
                v.push(size);
            }
        }
        v
    }
    pub fn all_sizes(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for (i, _) in self.arena.iter().enumerate() {
            v.push(self.size(i));
        }
        v
    }
    pub fn total_size(&self) -> u32 {
        self.size(0)
    }
}
