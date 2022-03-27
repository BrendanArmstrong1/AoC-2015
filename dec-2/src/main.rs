use std::fs;

fn main() {
    let file = fs::read_to_string("../input.txt").unwrap();
    part1(&file);
}

fn part1(file: &str) {
    for i in file.lines() {
        print!("{}", i);
    }
}
