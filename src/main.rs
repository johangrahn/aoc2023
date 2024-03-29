use std::collections::BTreeMap;
use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;

type DayFunction = fn(&str) -> (u32, u32);

fn main() {
    let mut map: BTreeMap<usize, DayFunction> = BTreeMap::new();

    map.insert(1, day1);
    map.insert(2, day2);
    map.insert(3, day3);

    map.keys().for_each(|key| {
        let function = map.get(key).unwrap();
        let input = read_to_string(format!("input/day{}.txt", key)).unwrap();
        println!("Day {}: {:?}", key, &function(&input))
    });
}
