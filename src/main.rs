mod puzzle;
mod day1;
mod day2;

use std::path::Path;

fn solve(day: Box<dyn puzzle::Puzzle>) {
    let (nr, name) = day.info();

    let example = format!("data/day{}.example", nr);
    let input = format!("data/day{}.input", nr);

    let example = Path::new(&example);
    let input = Path::new(&input);

    let puzzle_name = format!("Day {:02}: {}", nr, name);
    println!("--- {} ---", puzzle_name);

    let [expected, answer] = day.expected1();
    assert_eq!(day.solve1(example), expected, "{} (example)", puzzle_name);
    assert_eq!(day.solve1(input), answer, "{} (input)", puzzle_name);
    println!("    Answer: {}", answer);

    let puzzle_name = format!("Day {:02}: Part Two", nr);
    println!("--- {} ---", puzzle_name);

    let [expected, answer] = day.expected2();
    assert_eq!(day.solve2(example), expected, "{} (example)", puzzle_name);
    assert_eq!(day.solve2(input), answer, "{} (input)", puzzle_name);
    println!("    Answer: {}", answer);
}

fn main() {
    println!("\nAdvent of Code\n       λy.2022\n");

    let days: [Box<dyn puzzle::Puzzle>; 2] = [
        Box::new(day1::Day1 {}),
        Box::new(day2::Day2 {})
    ];

    for day in days {
        solve(day);
    }
}
