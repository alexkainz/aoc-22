use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day6;
use crate::puzzle::Puzzle;

fn solve(path: &Path, size: usize) -> usize {
    BufReader::new(File::open(path).unwrap())
        .lines().next().unwrap().unwrap().chars()
        .collect::<Vec<char>>()
        .windows(size)
        .position(|x| HashSet::<&char>::from_iter(x).len() == size)
        .unwrap() + size
}

impl Puzzle for Day6 {
    fn info(&self) -> (i8, String) { (6, String::from("Tuning Trouble")) }

    fn solve1(&self, path: &Path) -> String {
        solve(path, 4).to_string()
    }

    fn expected1(&self) -> [String; 2] { [7.to_string(), 1651.to_string()] }

    fn solve2(&self, path: &Path) -> String {
        solve(path, 14).to_string()
    }

    fn expected2(&self) -> [String; 2] { [19.to_string(), 3837.to_string()] }
}
