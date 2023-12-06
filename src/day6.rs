fn main() {
    let input = include_str!("../input/day6.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn get_numbers(prefix: &str, line: &str) -> Vec<u64> {
    line
        .strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>()
}

fn calculate_wins(races: Vec<(&u64, &u64)>) -> u64 {
    let mut wins: Vec<u64> = vec![];
    for (t, d) in races {
        let mut count = 0;
        for n in 0..=*t {
            if (t - n) * n > *d {
                count += 1;
            }
        }
        wins.push(count);
    }

    wins.iter().fold(1u64, |acc, r| acc * r)
}

fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let times = get_numbers("Time: ", lines[0]);
    let distances = get_numbers("Distance: ", lines[1]);
    let races = times.iter().
        zip(distances.iter()).collect::<Vec<_>>();

    calculate_wins(races)
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let time = lines[0]
        .strip_prefix("Time: ")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines[1]
        .strip_prefix("Distance: ")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let races = vec![(&time, &distance)];

    calculate_wins(races)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1_input() {
        let input_data = include_str!("../example/day6.txt");
        assert_eq!(part1(input_data), 288u64);
    }

    #[test]
    fn test_part_2_input() {
        let input_data = include_str!("../example/day6.txt");
        assert_eq!(part2(input_data), 71503u64);
    }
}