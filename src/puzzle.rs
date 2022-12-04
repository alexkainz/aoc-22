use std::path::Path;

pub trait Puzzle {
    fn info(&self) -> (i8, String);

    fn solve1(&self, _path: &Path) -> i32 { -1 }
    fn expected1(&self) -> [i32; 2] { [0, 0]}

    fn solve2(&self, _path: &Path) -> i32 { -1 }
    fn expected2(&self) -> [i32; 2] { [0, 0]}
}
