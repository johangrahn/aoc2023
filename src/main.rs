use std::collections::HashMap;
use std::fs::read_to_string;
mod day1;

use crate::day1::day1;

fn main() {
    let mut map = HashMap::new();
    let input = read_to_string("input/day1.txt").unwrap();

    map.insert(1, day1);

    if let Some(func) = map.get(&1) {
        let result = func(&input);
        println!("Day 1: {:?}", result);
    }
}
