pub fn day1(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>();

            // Get the first and last number
            (numbers[0] * 10) + numbers[numbers.len() - 1]
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line
                .chars()
                .enumerate()
                .map(|(index, c)| match c {
                    c if c.is_ascii_digit() => c.to_digit(10).unwrap_or(0),
                    _ => match &line[index..] {
                        s if s.starts_with("one") => 1,
                        s if s.starts_with("two") => 2,
                        s if s.starts_with("three") => 3,
                        s if s.starts_with("four") => 4,
                        s if s.starts_with("five") => 5,
                        s if s.starts_with("six") => 6,
                        s if s.starts_with("seven") => 7,
                        s if s.starts_with("eight") => 8,
                        s if s.starts_with("nine") => 9,
                        _ => 0,
                    },
                })
                .filter(|&num| num > 0)
                .collect();

            (numbers[0] * 10) + numbers[numbers.len() - 1]
        })
        .sum()
}

#[cfg(test)]
mod part1 {
    use std::fs::read_to_string;

    use crate::day1::part1;

    #[test]
    fn test_example() {
        let input = read_to_string("input/day1_example.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 142)
    }

    #[test]
    fn test_real_input() {
        let input = read_to_string("input/day1.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 55816)
    }
}

#[cfg(test)]
mod part2 {
    use std::fs::read_to_string;

    use crate::day1::part2;

    #[test]
    fn test_example() {
        let input = read_to_string("input/day1_example_part2.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 281)
    }

    #[test]
    fn test_real_input() {
        let input = read_to_string("input/day1.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 54980)
    }
}
