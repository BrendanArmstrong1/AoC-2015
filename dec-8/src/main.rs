use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    // part1(&file);
    part2(&file);
}

fn _part1(file: &str) {
    let mut acc = 0;

    for line in file.lines() {
        let mut count = 0;
        let mut ch = line.chars();

        while let Some(l) = ch.next() {
            if l == '\\' {
                match ch.next() {
                    Some(letter) if letter == 'x' => {
                        ch.next();
                        ch.next();
                        count += 1;
                    }
                    Some(_) => count += 1,
                    _ => {}
                }
            } else if l == '"' {
                continue;
            } else {
                count += 1;
            }
        }

        acc += line.len() - count;
        println!("{}, {}, {}", count, line, acc);
    }
    println!("{}", acc);
}

#[allow(unused_mut, unused)]
fn part2(file: &str) {
    let mut acc = 0;

    for line in file.lines() {
        let mut count = 0;
        let mut ch = line.chars();

        while let Some(l) = ch.next() {
            if l == '"' || l == '\\' {
                count += 2;
            } else {
                count += 1;
            }
        }
        count += 2;
        acc += count - line.len();
    }
    println!("{}", acc);
}
