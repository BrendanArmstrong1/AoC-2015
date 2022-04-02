use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    part1(&file);
}

fn part1(file: &str) {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for line in file.lines() {
        let arr: Vec<&str> = line.split(" -> ").collect();
        map.insert(arr[1], arr[0]);
    }

    // TODO this shit sucks, worst challenge so far. I'll do it later.
    for (_key, val) in map.iter() {
        let entry = val.split_whitespace().collect::<Vec<&str>>();
        let type_len = entry.len();
        let key = val.split_whitespace().collect::<Vec<&str>>()[0];
        println!("entry:{:?}, type_len:{}, key:{}", entry, type_len, key);
        if type_len == 3 {
            println!("{}", map[key]);
        } else if type_len == 2 {
            continue;
        }
    }
}
