use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve1(path: &Path) -> i32 {
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

    return cmp::max(elf_calories, max_calories);
}

fn solve2(path: &Path) -> i32 {
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

    return top_three[0] + top_three[1] + top_three[2];
}

pub fn run() {
    let example = Path::new("data/day1.example");
    let input = Path::new("data/day1.input");

    println!("--- Day 1: Calorie Counting ---");

    assert_eq!(solve1(example), 24000);

    let answer1 = solve1(input);
    assert_eq!(answer1, 70369);
    println!("Answer: {}", answer1);

    println!("--- Part Two ---");

    assert_eq!(solve2(example), 45000);
    let answer2 = solve2(input);
    assert_eq!(answer2, 203002);
    println!("Answer: {}", answer2);
}
