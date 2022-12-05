use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day1;
use crate::puzzle::Puzzle;

impl Puzzle for Day1 {
    fn info(&self) -> (i8, String) { (1, String::from("Calorie Counting")) }

    fn solve1(&self, path: &Path) -> String {
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);

        let mut elf_calories = 0;
        let mut max_calories = 0;

        for line in buf.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                max_calories = cmp::max(elf_calories, max_calories);
                elf_calories = 0;
            } else {
                elf_calories += line.parse::<i32>().unwrap();
            }
        }

        cmp::max(elf_calories, max_calories).to_string()
    }

    fn expected1(&self) -> [String; 2] { [24000.to_string(), 70369.to_string()] }

    fn solve2(&self, path: &Path) -> String {
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);

        let mut elf_calories = 0;
        let mut top_three = [0, 0, 0];

        fn next_top_three(top_three: [i32; 3], elf_calories: i32) -> [i32; 3] {
            return if elf_calories > top_three[0] {
                [elf_calories, top_three[0], top_three[1]]
            } else if elf_calories > top_three[1] {
                [top_three[0], elf_calories, top_three[1]]
            } else if elf_calories > top_three[2] {
                [top_three[0], top_three[1], elf_calories]
            } else {
                top_three
            }
        }

        for line in buf.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                top_three = next_top_three(top_three, elf_calories);
                elf_calories = 0;
            } else {
                elf_calories += line.parse::<i32>().unwrap();
            }
        }

        top_three = next_top_three(top_three, elf_calories);
        (top_three[0] + top_three[1] + top_three[2]).to_string()
    }

    fn expected2(&self) -> [String; 2] { [45000.to_string(), 203002.to_string()] }
}
