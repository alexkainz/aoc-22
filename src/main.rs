mod puzzle;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::path::Path;
use std::time::Instant;
use colored::Colorize;

fn solve(day: Box<dyn puzzle::Puzzle>) {
    let (nr, name) = day.info();

    let example = format!("data/day{}.example", nr);
    let input = format!("data/day{}.input", nr);

    let example = Path::new(&example);
    let input = Path::new(&input);

    let puzzle_name = format!("Day {:02}: {}", nr, name);
    println!("--- {} ---", puzzle_name.yellow());

    let [expected, answer] = day.expected1();
    assert_eq!(day.solve1(example), expected, "{} (example)", puzzle_name);

    let start = Instant::now();
    let expected = day.solve1(input);
    let duration = start.elapsed();

    assert_eq!(expected, answer, "{} (input)", puzzle_name);
    println!("    Answer: {} [{:?}]", answer.to_string().bold(), duration);

    let puzzle_name = format!("Day {:02}: Part Two", nr);
    println!("--- {} ---", puzzle_name.yellow());

    let [expected, answer] = day.expected2();
    assert_eq!(day.solve2(example), expected, "{} (example)", puzzle_name);

    let start = Instant::now();
    let expected = day.solve2(input);
    let duration = start.elapsed();

    assert_eq!(expected, answer, "{} (input)", puzzle_name);
    println!("    Answer: {} [{:?}]", answer.to_string().bold(), duration);
}

fn main() {
    println!("\n{}\n       {}\n", "Advent of Code".green(), "λy.2022".green());

    let days: [Box<dyn puzzle::Puzzle>; 5] = [
        Box::new(day1::Day1 {}),
        Box::new(day2::Day2 {}),
        Box::new(day3::Day3 {}),
        Box::new(day4::Day4 {}),
        Box::new(day5::Day5 {})
    ];

    for day in days {
        solve(day);
    }
}
