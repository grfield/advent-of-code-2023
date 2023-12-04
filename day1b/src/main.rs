const TOKENS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let input_data = include_str!("input_data.txt");
    println!("Day 1b answer: {}", calculate_sum_from_codes(input_data));
}

fn calculate_sum_from_codes(input: &str) -> u32 {
    input.lines()
        .map(|l| { number_from_first_and_last_digit(l)})
        .sum::<u32>()
}

fn find_token_match(s: &str) -> Option<&str> {
    for t in TOKENS {
        if s.starts_with(t) {
            return Some(t)
        }
    }

    None
}

fn get_char_for_token(token: &str) -> char {
    let index = TOKENS.iter().position(|&r| r == token).unwrap();
    char::from_digit(index as u32 + 1, 10).unwrap()
}

fn find_digits_or_words(line: &str) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();

    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if let Some(t) = find_token_match(&line[i..]) {
            digits.push(get_char_for_token(t));
        }
    }

    digits
}

fn number_from_first_and_last_digit(line: &str) -> u32 {
    let digits = find_digits_or_words(line);
    let number_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    number_str.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::number_from_first_and_last_digit;
    use crate::calculate_sum_from_codes;

    #[test]
    fn two_number_words_end_to_end_in_str() {
        let input = "twonine";
        assert_eq!(number_from_first_and_last_digit(input), 29u32);
    }

    #[test]
    fn two_number_words_ending_with_digit_in_str() {
        let input = "threehqv2";
        assert_eq!(number_from_first_and_last_digit(input), 32u32);
    }

    #[test]
    fn two_number_words_overlapping_in_str() {
        let input = "twone";
        assert_eq!(number_from_first_and_last_digit(input), 21u32);
    }

    #[test]
    fn test_input_file_2() {
        let input_data = include_str!("testinput.txt");
        assert_eq!(calculate_sum_from_codes(input_data), 281u32);
    }
}