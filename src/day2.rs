use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

trait Score {
    fn score(&self) -> i32;
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape { Rock, Paper, Scissors }

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Shape, Self::Err> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _         => Err(()),
        }
    }
}

impl Score for Shape {
    fn score(&self) -> i32 {
        match *self { Shape::Rock => 1, Shape::Paper => 2, Shape::Scissors => 3 }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome { Lost, Draw, Won }

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lost),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Won),
            _         => Err(()),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        match *self { Outcome::Lost => 0, Outcome::Draw => 3, Outcome::Won => 6 }
    }
}

fn play(a: Shape, b: Shape) -> Outcome {
    if a == b { Outcome::Draw }
    else if a == Shape::Rock && b == Shape::Scissors { Outcome::Won }
    else if a == Shape::Scissors && b == Shape::Paper { Outcome::Won }
    else if a == Shape::Paper && b == Shape::Rock { Outcome::Won }
    else { Outcome::Lost }
}

fn choose(shape: Shape, outcome: Outcome) -> Shape {
    if outcome == Outcome::Draw { shape }
    else {
        match shape {
            Shape::Rock => if outcome == Outcome::Won { Shape::Paper } else { Shape::Scissors },
            Shape::Scissors => if outcome == Outcome::Won { Shape::Rock } else { Shape::Paper },
            Shape::Paper => if outcome == Outcome::Won { Shape::Scissors } else { Shape::Rock }
        }
    }
}

fn solve1(path: &Path) -> i32 {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    let mut total_score = 0;

    for line in buf.lines() {
        let line = line.unwrap();
        let mut it = line.split_whitespace();
        let line = [it.next().unwrap(), it.next().unwrap()];
        let a = Shape::from_str(line[0]).unwrap();
        let b = Shape::from_str(line[1]).unwrap();

        total_score += b.score() + play(b, a).score();
    }

    return total_score;
}

fn solve2(path: &Path) -> i32 {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    let mut total_score = 0;

    for line in buf.lines() {
        let line = line.unwrap();
        let mut it = line.split_whitespace();
        let line = [it.next().unwrap(), it.next().unwrap()];
        let a = Shape::from_str(line[0]).unwrap();
        let outcome = Outcome::from_str(line[1]).unwrap();
        let b = choose(a, outcome);

        total_score += b.score() + play(b, a).score();
    }

    return total_score;
}

pub fn run() {
    let example = Path::new("data/day2.example");
    let input = Path::new("data/day2.input");

    println!("--- Day 2: Rock Paper Scissors ---");

    assert_eq!(solve1(example), 15);

    let answer1 = solve1(input);
    assert_eq!(answer1, 13526);
    println!("Answer: {}", answer1);

    println!("--- Part Two ---");

    assert_eq!(solve2(example), 12);
    let answer2 = solve2(input);
    assert_eq!(answer2, 14204);
    println!("Answer: {}", answer2);
}
