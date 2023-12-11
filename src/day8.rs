use std::collections::HashMap;

#[derive(Debug)]
struct Map<'a> {
    left: &'a str,
    right: &'a str
}

fn main() {
    let input = include_str!("../input/day8.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn part1(input: &str) -> u32 {
    let (route_str, maps) = input.split_once("\n\n").unwrap();
    let route = route_str.chars().collect::<Vec<char>>();

    let mut map: HashMap<&str, Map> = HashMap::new();
    for path in maps.lines() {
        let (key, value) = path.split_once('=').unwrap();
        let (left_str, right_str) = value.split_once(',').unwrap();
        let (_, left) = left_str.split_once('(').unwrap();
        let (right, _) = right_str.split_once(')').unwrap();
        map.insert(key.trim(), Map { left, right: right.trim_start() });
    };

    let mut position = "AAA";
    let mut steps = 0;
    while position != "ZZZ" {
        for step in &route {
            let instruction: &Map = map.get(position).unwrap();
            position = match step {
                'L' => instruction.left,
                'R' => instruction.right,
                _ => panic!()
            };

            steps += 1;
        }
    }

    steps
}

fn part2(_input: &str) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1a_input() {
        let input_data = include_str!("../example/day8a.txt");
        assert_eq!(part1(input_data), 2);
    }

    #[test]
    fn test_part_1b_input() {
        let input_data = include_str!("../example/day8b.txt");
        assert_eq!(part1(input_data), 6);
    }

    #[test]
    fn test_part_2_input() {
        let input_data = include_str!("../example/day8a.txt");
        assert_eq!(part2(input_data), 0);
    }
}