struct Card {
    winning_nums: Vec<u32>,
    entry: Vec<u32>
}

struct CardStats {
    matches: u32,
    count: u32
}

fn main() {
    let input = include_str!("../input/day4.txt");
    advent_of_code_2023::solve_puzzles(input, part1, part2);
}

fn part1(input: &str) -> u32 {
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

fn part2(input: &str) -> u32 {
    // build hand of cards
    let mut cards: Vec<CardStats> = Vec::new();
    for s in input.lines() {
        let card = parse_card_stats(s);
        cards.push(card);
    }

    // apply scratch card rules one by one
    for i in 0..cards.len() {
        let current_copies = cards[i].count;
        let card_id = (i + 1) as u32;
        let matches = cards[i].matches;
        for j in card_id + 1..card_id + 1 + matches {
            let card = cards.get_mut(j as usize - 1).unwrap();
            card.count += current_copies;
        }
    }

    cards.iter().map(|c| c.count).sum()
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

fn parse_card_stats(card: &str) -> CardStats {
    let data: Vec<_> = card.split(&[':', '|']).collect();

    let winning: Vec<u32> = data[1].split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    let matching_nums: Vec<u32> = data[2].split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .filter(|n| winning.contains(n))
        .collect();

    CardStats { matches: matching_nums.len() as u32, count: 1 }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_score() {
        let input_data = include_str!("../example/day4.txt");
        assert_eq!(part1(input_data), 13u32);
    }

    #[test]
    fn part2_total_scratchcards() {
        let input_data = include_str!("../example/day4.txt");
        assert_eq!(part2(input_data), 30u32);
    }
}