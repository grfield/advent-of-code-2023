struct Card {
    winning_nums: Vec<u32>,
    entry: Vec<u32>
}

fn main() {
    let input_data = include_str!("input_data.txt");
    println!("Day 4a answer: {}", winning_score_for_cards(input_data));
}

fn parse_card(card: &str) -> Card {
    let data: Vec<_> = card.split(&[':', '|']).collect();
    let winning: Vec<u32> = data[1].split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    let entry: Vec<u32> = data[2].split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    Card {
        winning_nums: winning,
        entry,
    }
}

fn winning_score_for_cards(input: &str) -> u32 {
    let mut sum = 0;
    for s in input.lines() {
        let card = parse_card(s);
        let matching_nums: Vec<_> = card.entry.iter()
            .filter(|c| card.winning_nums.contains(c))
            .collect();
        if !matching_nums.is_empty() {
            sum += 2u32.pow(matching_nums.len() as u32 - 1)
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::winning_score_for_cards;

    #[test]
    fn test_input_file_1() {
        let input_data = include_str!("testinput.txt");
        assert_eq!(winning_score_for_cards(input_data), 13u32);
    }
}