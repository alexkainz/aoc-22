use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day4;
use crate::puzzle::Puzzle;

fn get_pairs(line: &String) -> [[i32; 2]; 2] {
    fn get_pair(s: &str) -> [i32; 2] {
        let mut it = s.split('-');
        [it.next().unwrap().parse::<i32>().unwrap(), it.next().unwrap().parse::<i32>().unwrap()]
    }

    let mut it = line.split(',');
    [get_pair(it.next().unwrap()), get_pair(it.next().unwrap())]
}

impl Puzzle for Day4 {
    fn info(&self) -> (i8, String) { (4, String::from("Camp Cleanup")) }

    fn solve1(&self, path: &Path) -> i32 {
        let buf = BufReader::new(File::open(path).unwrap());
        let mut pairs = 0;

        fn contains(a: [i32; 2], b: [i32; 2]) -> bool {
            a[0] >= b[0] && a[1] <= b[1]
        }

        for line in buf.lines() {
            let line = line.unwrap();
            let [first, second] = get_pairs(&line);

            if contains(first, second) || contains(second, first) {
                pairs += 1;
            }
        }

        pairs
    }

    fn expected1(&self) -> [i32; 2] { [2, 305] }

    fn solve2(&self, path: &Path) -> i32 {
        let buf = BufReader::new(File::open(path).unwrap());
        let mut pairs = 0;

        fn overlaps(a: [i32; 2], b: [i32; 2]) -> bool {
            a[0] >= b[0] && a[0] <= b[1] || a[1] >= b[0] && a[1] <= b[1]
        }

        for line in buf.lines() {
            let line = line.unwrap();
            let [first, second] = get_pairs(&line);

            if overlaps(first, second) || overlaps(second, first) {
                pairs += 1;
            }
        }

        pairs
    }

    fn expected2(&self) -> [i32; 2] { [4, 811] }
}
