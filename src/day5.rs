fn main() {
    let input = include_str!("../input/day5.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn part1(input: &str) -> u32 {
    todo!()
}

fn part2(input: &str) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1_input() {
        let input_data = include_str!("../example/day5.txt");
        assert_eq!(part1(input_data), ?);
    }

    #[test]
    fn test_part_2_input() {
        let input_data = include_str!("../example/day5.txt");
        assert_eq!(part1(input_data), ?);
    }
}