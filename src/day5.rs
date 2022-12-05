use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
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
        // println!("push {} on stack {}", c, stack);
        if stack+1 > self.stacks.len() { self.stacks.resize(stack+1, Vec::new()); }
        self.stacks[stack].insert(0, c);
    }

    fn move_to(&mut self, from: usize, to: usize) {
        // println!("move {} to {}", from, to);
        let c = self.stacks[from].pop().unwrap();
        self.stacks[to].push(c);
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

impl Puzzle for Day5 {
    fn info(&self) -> (i8, String) { (5, String::from("Supply Stacks")) }

    fn solve1(&self, path: &Path) -> String {
        let buf = BufReader::new(File::open(path).unwrap());
        let mut cargo = Cargo::new();

        for line in buf.lines() {
            let line = line.unwrap();

            if line.is_empty() { continue; }

            if line.starts_with("move") {
                // println!("message: {}", cargo.message());
                let mut it = line.split_whitespace();
                it.next(); // skip move
                let quantity = it.next().unwrap().parse::<usize>().unwrap();
                it.next(); // skip from
                let from = it.next().unwrap().parse::<usize>().unwrap();
                it.next(); // skip to
                let to = it.next().unwrap().parse::<usize>().unwrap();

                for _ in 0..quantity { cargo.move_to(from-1, to-1); }

                continue;
            }

            let mut i = 0;
            for c in line.chars() {
                if c.is_alphabetic() && i % 4 == 1 {
                    cargo.push(c, i / 4);
                    // println!("message: {}", cargo.message());
                }
                i += 1;
            }
        }

        cargo.message()
    }

    fn expected1(&self) -> [String; 2] { [String::from("CMZ"), String::from("TGWSMRBPN")] }

    fn solve2(&self, path: &Path) -> String {
        String::new()
    }

    fn expected2(&self) -> [String; 2] { [String::from("MCD"), String::from("MCD")] }
}
