const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

fn main() {
    let input = include_str!("../input/day2.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn parse_games(input: &str) -> Vec<Vec<&str>> {
    input.lines()
        .map(|s| { s. split(&[':', ';']).collect::<Vec<_>>() })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    let games = parse_games(input);

    let mut possible_game_id_total: u32 = 0;
    for game in games {
        let (_, game_num_str) = game[0].split_once(' ').unwrap();
        let game_num = game_num_str.parse::<u32>().unwrap();
        let mut game_possible = true;
        for draw_idx in 1..game.len() {
            let balls = game[draw_idx].split(',').collect::<Vec<_>>();
            for draw in balls {
                let colour_balls = draw.split_ascii_whitespace().collect::<Vec<_>>();
                let ball_colour = colour_balls[1];
                let ball_count = colour_balls[0].parse::<u32>().unwrap();
                if ball_colour == "red" && ball_count > RED_COUNT {
                    game_possible = false;
                }
                if ball_colour == "green" && ball_count > GREEN_COUNT {
                    game_possible = false;
                }
                if ball_colour == "blue" && ball_count > BLUE_COUNT {
                    game_possible = false;
                }
            }
        }
        if game_possible { possible_game_id_total += game_num }
    }

    possible_game_id_total
}

fn part2(input: &str) -> u32 {
    let games = parse_games(input);

    let mut sum = 0u32;
    for game in games {
        let mut ball_power = 0u32;
        let mut max_red = 0u32;
        let mut max_green = 0u32;
        let mut max_blue = 0u32;
        for draw_idx in 1..game.len() {
            let balls = game[draw_idx].split(',').collect::<Vec<_>>();
            for draw in balls {
                let colour_balls = draw.split_ascii_whitespace().collect::<Vec<_>>();
                let ball_colour = colour_balls[1];
                let ball_count = colour_balls[0].parse::<u32>().unwrap();
                if ball_colour == "red" && ball_count > max_red {
                    max_red = ball_count;
                }
                if ball_colour == "green" && ball_count > max_green {
                    max_green = ball_count;
                }
                if ball_colour == "blue" && ball_count > max_blue {
                    max_blue = ball_count;
                }
            }
            ball_power = max_red * max_green * max_blue;
        }
        sum += ball_power;
    }

    sum
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1_input() {
        let input_data = include_str!("../example/day2.txt");
        assert_eq!(part1(input_data), 8);
    }

    #[test]
    fn test_part_2_input() {
        let input_data = include_str!("../example/day2.txt");
        assert_eq!(part2(input_data), 2286);
    }
}