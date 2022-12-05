use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day5;
use crate::puzzle::Puzzle;

struct Cargo {
    stacks: Vec<Vec<char>>
}

impl Cargo {
    fn new() -> Cargo {
        Cargo { stacks: Vec::new() }
    }

    fn push(&mut self, c: char, stack: usize) {
        if stack+1 > self.stacks.len() { self.stacks.resize(stack+1, Vec::new()); }
        self.stacks[stack].insert(0, c);
    }

    fn move_to(&mut self, quantity: usize, from: usize, to: usize, crane: u16) {
        let stack_len = self.stacks[to].len();

        for _ in 0..quantity {
            let c = self.stacks[from].pop().unwrap();
            if crane == 9000 {
                self.stacks[to].push(c);
            } else {
                self.stacks[to].insert(stack_len, c);
            }
        }
    }

    fn message(&self) -> String {
        let mut message = String::new();
        let default = ' ';

        for stack in &self.stacks {
            let c = stack.last().unwrap_or(&default);
            message.push(*c);
        }

        message
    }
}

fn solve(path: &Path, crane: u16) -> String {
    let buf = BufReader::new(File::open(path).unwrap());
    let mut cargo = Cargo::new();

    for line in buf.lines() {
        let line = line.unwrap();

        if line.is_empty() { continue; }

        if line.starts_with("move") {
            let mut it = line.split_whitespace();
            it.next(); // skip move
            let quantity = it.next().unwrap().parse::<usize>().unwrap();
            it.next(); // skip from
            let from = it.next().unwrap().parse::<usize>().unwrap();
            it.next(); // skip to
            let to = it.next().unwrap().parse::<usize>().unwrap();

            cargo.move_to(quantity, from-1, to-1, crane);

            continue;
        }

        let mut i = 0;
        for c in line.chars() {
            if c.is_alphabetic() && i % 4 == 1 { cargo.push(c, i / 4); }
            i += 1;
        }
    }

    cargo.message()
}

impl Puzzle for Day5 {
    fn info(&self) -> (i8, String) { (5, String::from("Supply Stacks")) }

    fn solve1(&self, path: &Path) -> String { solve(path, 9000) }

    fn expected1(&self) -> [String; 2] { [String::from("CMZ"), String::from("TGWSMRBPN")] }

    fn solve2(&self, path: &Path) -> String { solve(path, 9001) }

    fn expected2(&self) -> [String; 2] { [String::from("MCD"), String::from("TZLTLWRNF")] }
}
