fn main() {
    println!("Hello, world!");
}

fn part1(input: String) -> u32 {
    println!("{input}");
    
    input.lines().map(|line| {
        let numbers: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).map(|c|c.to_digit(10).unwrap_or(0)).collect::<Vec<_>>();
    (numbers[0] * 10) + numbers.last().unwrap() 
    }).sum()
}

fn part2(input: String) -> u32 {
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

        #[test]
        fn test_real_input() {
            let input = read_to_string("input/day1.txt").unwrap();
            let result = part1(input);
            assert_eq!(result, 55816)
        }
    }

    mod part2 {
        use std::fs::read_to_string;

        use crate::part2;

        #[test]
        fn test_example() {
            let input = read_to_string("input/day1_example_part2.txt").unwrap();
            let result = part2(input);
            assert_eq!(result, 281)
        }

        #[test]
        fn test_real_input() {
            let input = read_to_string("input/day1.txt").unwrap();
            let result = part2(input);
            assert_eq!(result, 54980)
        }
    }
}

