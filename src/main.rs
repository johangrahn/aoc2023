use std::fs::read_to_string;
mod day1;

use day1::part1;
use day1::part2;

fn main() {
    let input = read_to_string("input/day1.txt").unwrap();
    let result = (part1(&input), part2(&input));
    println!("Day 1: {:?}", result);
}
