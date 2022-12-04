use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day3;
use crate::puzzle::Puzzle;

const CAPITAL_A: i32 = 65;
const SMALL_A: i32 = 97;

fn priority(codepoint: i32) -> i32 {
    if codepoint >= SMALL_A { codepoint - SMALL_A + 1 } else { codepoint - CAPITAL_A + 27 }
}

impl Puzzle for Day3 {
    fn info(&self) -> (i8, String) { (3, String::from("Rucksack Reorganization")) }

    fn solve1(&self, path: &Path) -> i32 {
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);

        let mut sum_priorities = 0;

        for line in buf.lines() {
            let line = line.unwrap();
            let (first, second) = line.split_at(line.len() / 2);
            let first = String::from(first);
            let common = second.chars()
                .find(|c| first.contains(*c))
                .unwrap() as i32;

            sum_priorities += priority(common);
        }

        sum_priorities
    }

    fn expected1(&self) -> [i32; 2] { [157, 8153] }

    fn solve2(&self, path: &Path) -> i32 {
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);

        let mut sum_priorities = 0;
        let mut group: [String; 2] = [ String::new(), String::new() ];

        for (i, line) in buf.lines().enumerate() {
            let line = line.unwrap();
            let elf_in_group = i % 3;

            if elf_in_group != 2 {
                group[elf_in_group] = line;
                continue;
            }

            let common = line.chars()
                .find(|c| group[0].contains(*c) && group[1].contains(*c))
                .unwrap() as i32;

            sum_priorities += priority(common);
        }

        sum_priorities
    }

    fn expected2(&self) -> [i32; 2] { [70, 2342] }
}
