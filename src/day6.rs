use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Day6;
use crate::puzzle::Puzzle;

impl Puzzle for Day6 {
    fn info(&self) -> (i8, String) { (6, String::from("Tuning Trouble")) }

    /*
    fn solve1(&self, path: &Path) -> String {
        let buf = BufReader::new(File::open(path).unwrap());
        let line = buf.lines().next().unwrap().unwrap();
        let mut chars = line.chars();
        let recent_four: [char; 4] = [chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap()];

        println!("{:?}", recent_four);

        "".to_string()
    }

     */

    fn solve1(&self, path: &Path) -> String {
        let buf = BufReader::new(File::open(path).unwrap());
        let line = buf.lines().next().unwrap().unwrap();
        let vec = line.chars().collect::<Vec<char>>();

        let mut it = vec.windows(4);
        // println!("{:?}", it.next().unwrap());
        // println!("{:?}", it.next().unwrap());

        // let a = it.position(|x| HashSet::from(x).len() == 4);

        // it.for_each(|x| println!("{:?}", x));
        /*
        it
            .for_each(|x| {
                println!("{:?}", HashSet::<&char>::from_iter(x));
        });

         */

        (4 + it
            .position(|x| HashSet::<&char>::from_iter(x).len() == 4)
            .unwrap())
            .to_string()
    }

    fn expected1(&self) -> [String; 2] { [7.to_string(), 1651.to_string()] }

    fn solve2(&self, path: &Path) -> String {
        "".to_string()
    }

    fn expected2(&self) -> [String; 2] { [String::from("MCD"), String::from("TZLTLWRNF")] }
}
