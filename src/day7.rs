use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day7;
use crate::puzzle::Puzzle;

struct INode {
    size: usize,
    parent: Option<usize>,
    children: Option<HashMap<String, usize>>
}

impl INode {
    fn parent(&self) -> usize {
        self.parent.unwrap()
    }

    fn child(&self, name: &str) -> usize {
        *self.children.as_ref().unwrap().get(name).unwrap()
    }

    fn add_child(&mut self, name: &str, inumber: usize) {
        self.children.as_mut().unwrap().insert(String::from(name), inumber);
    }

    fn is_directory(&self) -> bool {
        self.children.is_some()
    }

    fn size(&self, filetable: &Vec<INode>) -> usize {
        if self.children.is_none() {
            self.size
        } else {
            self.children.as_ref().unwrap().iter()
                .map(|(_, inumber)| {
                    filetable[*inumber].size(filetable)
                })
                .sum()
        }
    }
}

struct Filesystem {
    filetable: Vec<INode>,
    root: usize,
    current_directory: usize
}

impl Filesystem {
    fn cd(&mut self, directory: &str) {
        let current_directory = &self.filetable[self.current_directory];

        self.current_directory = if directory.eq("..") {
            current_directory.parent()
        } else if directory.eq("/") {
            self.root
        } else {
            current_directory.child(directory)
        }
    }

    fn add_file(&mut self, size: usize, name: &str) {
        let child_id = self.filetable.len();
        let current_directory = &mut self.filetable[self.current_directory];

        current_directory.add_child(name, child_id);

        self.filetable.push(INode {
            size,
            parent: None,
            children: None
        });
    }

    fn add_directory(&mut self, name: &str) {
        let child_id = self.filetable.len();
        let current_directory = &mut self.filetable[self.current_directory];

        current_directory.add_child(name, child_id);

        self.filetable.push(INode {
            size : 0,
            parent: Some(self.current_directory),
            children: Some(HashMap::new())
        });
    }

    fn new() -> Filesystem {
        Filesystem {
            filetable: vec![INode {
                size: 0,
                parent: None,
                children: Some(HashMap::new())
            }],
            root: 0,
            current_directory: 0
        }
    }

    fn parse(path: &Path) -> Filesystem {
        let mut filesystem = Filesystem::new();

        BufReader::new(std::fs::File::open(path).unwrap())
            .lines()
            .for_each(|line| {
                let line = line.unwrap();

                if line.starts_with("$ cd ") {
                    filesystem.cd(&line[5..]);
                } else if line.starts_with("$ ls") {
                    // ignore the line
                } else if line.starts_with("dir ") {
                    filesystem.add_directory(&line[4..]);
                } else {
                    let mut it = line.split_whitespace();
                    filesystem.add_file(it.next().unwrap().parse().unwrap(), it.next().unwrap());
                }
            });

        filesystem
    }
}


impl Puzzle for Day7 {
    fn info(&self) -> (i8, String) { (7, String::from("No Space Left On Device")) }

    fn solve1(&self, path: &Path) -> String {
        let filesystem = Filesystem::parse(path);
        let filetable = &filesystem.filetable;

        let sum: usize = filetable.iter()
            .filter(|x| { x.is_directory() })
            .map(|x| { x.size(filetable) })
            .filter(|x| { *x <= 100000 })
            .sum();

        sum.to_string()
    }

    fn expected1(&self) -> [String; 2] { [95437.to_string(), 1297683.to_string()] }

    fn solve2(&self, path: &Path) -> String {
        let filesystem = Filesystem::parse(path);
        let filetable = &filesystem.filetable;

        let total_space = 70000000;
        let space_needed = 30000000;
        let space_used = filetable[0].size(filetable);
        let to_delete = space_needed - (total_space - space_used);

        let min: usize = filetable.iter()
            .filter(|x| { x.is_directory() })
            .map(|x| { x.size(filetable) })
            .filter(|x| { *x >= to_delete })
            .min()
            .unwrap();

        min.to_string()
    }

    fn expected2(&self) -> [String; 2] { [24933642.to_string(), 5756764.to_string()] }
}
