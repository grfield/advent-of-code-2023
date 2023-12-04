const BALL_GROUP: BallGroup = BallGroup { red: 12, green: 13, blue: 14 };

struct BallGroup {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let input_data = include_str!("input_data.txt");
    let bag_contents = BallGroup { red: 12, green: 13, blue: 14 };
    println!("Day 2a answer: {}", calculate_sum_of_valid_game_ids(input_data, bag_contents));
    //println!("Day 2b answer: {}", calculate_sum_of_game_ids(input_data, bag_contents));
}

fn calculate_sum_of_valid_game_ids(input: &str, _bag_contents: BallGroup) -> u32 {
    let lines = input.lines();
    for line in lines {
        line.split(&[':', ';'])
    }
        .map(|l| {
            l.split(&[':', ';'])
                .map(str::to_owned)
                .collect::<Vec<_>>()
        } )
        .collect();
    println!("{:?}", output);

    2
}

#[cfg(test)]
mod tests {
    use crate::{BALL_GROUP, calculate_sum_of_valid_game_ids};

    #[test]
    fn test_input_file_1() {
        let input_data = include_str!("testinput_1.txt");
        assert_eq!(calculate_sum_of_valid_game_ids(input_data, BALL_GROUP), 8);
    }
}