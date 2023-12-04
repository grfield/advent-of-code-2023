#[derive(Debug)]
struct Card {
    matches: u32,
    count: u32
}

fn main() {
    let input_data = include_str!("input_data.txt");
    println!("Day 4a answer: {} scratch cards", total_scratchcards(input_data));
}

fn parse_card(card: &str) -> Card {
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

    Card { matches: matching_nums.len() as u32, count: 1 }
}

fn total_scratchcards(input: &str) -> u32 {
    // build hand of cards
    let mut cards: Vec<Card> = Vec::new();
    for s in input.lines() {
        let card = parse_card(s);
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

#[cfg(test)]
mod tests {
    use crate::total_scratchcards;

    #[test]
    fn test_input_file_1() {
        let input_data = include_str!("testinput.txt");
        assert_eq!(total_scratchcards(input_data), 303u32);
    }
}