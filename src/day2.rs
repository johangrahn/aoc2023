use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let config = HashMap::from([("red", 12), ("blue", 14), ("green", 13)]);

    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let game = line.split(':').last().unwrap();
            let false_games = game
                .split([';', ','])
                .map(|rev| rev.trim())
                .map(|rev| {
                    let s = rev.split_whitespace().collect::<Vec<_>>();
                    let number: usize = s[0].parse().unwrap();
                    let t = s[1];

                    let mapped_value = config.get(t).unwrap();
                    number <= *mapped_value
                })
                .filter(|v| !(*v))
                .collect::<Vec<_>>();

            if false_games.is_empty() {
                index as u32 + 1
            } else {
                0
            }
        })
        .sum::<_>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut map: HashMap<&str, u32> = HashMap::new();
            let game = line.split(':').last().unwrap();
            game.split([';', ','])
                .map(|rev| rev.trim())
                .for_each(|rev| {
                    let s = rev.split_whitespace().collect::<Vec<_>>();
                    let mut number: u32 = s[0].parse().unwrap();
                    let t = s[1];

                    map.entry(t)
                        .and_modify(|val| {
                            if val < &mut number {
                                *val = number
                            }
                        })
                        .or_insert(number);
                });

            map.values().product::<u32>()
        })
        .sum::<_>()
}
#[cfg(test)]
mod part1 {
    use std::fs::read_to_string;

    use crate::day2::part1;

    #[test]
    fn test_example() {
        let input = read_to_string("input/day2_example.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 8)
    }

    #[test]
    fn test_real_input() {
        let input = read_to_string("input/day2.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 2207)
    }
}
#[cfg(test)]
mod part2 {
    use std::fs::read_to_string;

    use crate::day2::part2;

    #[test]
    fn test_example() {
        let input = read_to_string("input/day2_example.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 2286)
    }

    #[test]
    fn test_real_input() {
        let input = read_to_string("input/day2.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 62241)
    }
}
