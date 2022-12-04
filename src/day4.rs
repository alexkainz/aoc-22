use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day4;
use crate::puzzle::Puzzle;

impl Puzzle for Day4 {
    fn info(&self) -> (i8, String) { (4, String::from("Camp Cleanup")) }

    fn solve1(&self, path: &Path) -> i32 {
        let buf = BufReader::new(File::open(path).unwrap());
        let mut containments = 0;

        fn get_pair(s: &str) -> [i32; 2] {
            let mut it = s.split('-');
            [it.next().unwrap().parse::<i32>().unwrap(), it.next().unwrap().parse::<i32>().unwrap()]
        }

        for line in buf.lines() {
            let line = line.unwrap();
            let mut it = line.split(',');
            let first = get_pair(it.next().unwrap());
            let second = get_pair(it.next().unwrap());

            if first[0] >= second[0] && first[1] <= second[1] || first[0] <= second[0] && first[1] >= second[1] {
                containments += 1;
            }
        }

        return containments;
    }

    fn expected1(&self) -> [i32; 2] { [2, 305] }


    fn expected2(&self) -> [i32; 2] { [70, 2342] }
}
