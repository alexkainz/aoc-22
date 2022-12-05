use std::path::Path;

pub trait Puzzle {
    fn info(&self) -> (i8, String);

    fn solve1(&self, _path: &Path) -> String { String::new() }
    fn expected1(&self) -> [String; 2] { [String::new(), String::new()]}

    fn solve2(&self, _path: &Path) -> String { String::new() }
    fn expected2(&self) -> [String; 2] { [String::new(), String::new()]}
}
