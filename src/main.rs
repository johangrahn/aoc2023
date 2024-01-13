fn main() {
    println!("Hello, world!");
}

fn part1(input: String) -> u64 {
    println!("{input}");
    0

}

#[cfg(test)]
mod tests {
    mod part1 {
        use std::fs::read_to_string;

        use crate::part1;

        #[test]
        fn test_example() {
            
            let input = read_to_string("input/day1_example.txt").unwrap();
            let result = part1(input);
            assert_eq!(result, 142)
        }

        
    }
}

