use std::collections::HashMap;

#[derive(Debug)]
struct Map<'a> {
    left: &'a str,
    right: &'a str
}

type MapType<'a> = HashMap<&'a str, Map<'a>>;

fn main() {
    let input = include_str!("../input/day8.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn parse_route_and_map(input: &str) -> (Vec<char>, MapType) {
    let (route_str, maps) = input.split_once("\n\n").unwrap();
    let route = route_str.chars().collect::<Vec<char>>();

    let mut map: MapType = HashMap::new();
    for path in maps.lines() {
        let (key, value) = path.split_once('=').unwrap();
        let (left_str, right_str) = value.split_once(',').unwrap();
        let (_, left) = left_str.split_once('(').unwrap();
        let (right, _) = right_str.split_once(')').unwrap();
        map.insert(key.trim(), Map { left, right: right.trim_start() });
    };

    (route, map)
}

fn next_node_location<'a>(instruction: &char, map: &'a MapType, node: &str) -> &'a str {
    let directions: &Map = map.get(node).unwrap();

    match instruction {
        'L' => directions.left,
        'R' => directions.right,
        _ => panic!()
    }
}

fn lcm (nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn part1(input: &str) -> u32 {
    let (route, map) = parse_route_and_map(input);

    let mut node = "AAA";
    let mut counter = 0;
    let mut steps = 0;
    while !node.ends_with('Z') {
        node = next_node_location(&route[counter], &map, node);
        counter = (counter + 1) % route.len();
        steps += 1;
    }

    steps
}

fn part2(input: &str) -> u64 {
    let (route, map) = parse_route_and_map(input);

    let nodes = map.keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<&str>>();
    let mut ghost_steps: Vec<u64> = Vec::new();
    for n in nodes {
        let mut node = n;
        let mut counter = 0;
        let mut steps = 0;
        while !node.ends_with('Z') {
            node = next_node_location(&route[counter], &map, node);
            counter = (counter + 1) % route.len();
            steps += 1;
        }

        ghost_steps.push(steps);
    }

    let boxed: Box<[u64]> = ghost_steps.into_boxed_slice();
    lcm(&boxed)
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1a_input() {
        let input_data = include_str!("../example/day8a_part1.txt");
        assert_eq!(part1(input_data), 2);
    }

    #[test]
    fn test_part_1b_input() {
        let input_data = include_str!("../example/day8b_part1.txt");
        assert_eq!(part1(input_data), 6);
    }

    #[test]
    fn test_part_2_input() {
        let input_data = include_str!("../example/day8_part2.txt");
        assert_eq!(part2(input_data), 6);
    }
}