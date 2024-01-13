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

