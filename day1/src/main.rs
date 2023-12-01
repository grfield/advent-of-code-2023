const TOKENS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let input_data = include_str!("input_data.txt");
    println!("Day 1a answer: {}", calculate_sum_from_codes(input_data, false));
    println!("Day 1b answer: {}", calculate_sum_from_codes(input_data, true));
}

fn calculate_sum_from_codes(input: &str, include_words: bool) -> u32 {
    input.lines()
        .map(|l| { number_from_first_and_last_digit(l, include_words)})
        .reduce(|a, b| a + b).unwrap()
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

fn find_digits_or_words(line: &str, include_words: bool) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();

    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if include_words {
            if let Some(t) = find_token_match(&line[i..]) {
                digits.push(get_char_for_token(t));
            }
        }
    }

    digits
}

fn number_from_first_and_last_digit(line: &str, include_words: bool) -> u32 {
    let digits = find_digits_or_words(line, include_words);
    let number_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    number_str.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::number_from_first_and_last_digit;
    use crate::calculate_sum_from_codes;

    #[test]
    fn only_two_digits_in_str() {
        let input = "bb5fdwDvj6V";
        assert_eq!(number_from_first_and_last_digit(input, false), 56u32);
    }

    #[test]
    fn only_one_digit_in_str() {
        let input = "z1dwDvjBgV";
        assert_eq!(number_from_first_and_last_digit(input, false), 11u32);
    }

    #[test]
    fn test_input_file_1() {
        let input_data = include_str!("testinput_1.txt");
        assert_eq!(calculate_sum_from_codes(input_data, false), 142u32);
    }

    #[test]
    fn two_number_words_end_to_end_in_str() {
        let input = "twonine";
        assert_eq!(number_from_first_and_last_digit(input, true), 29u32);
    }

    #[test]
    fn two_number_words_ending_with_digit_in_str() {
        let input = "threehqv2";
        assert_eq!(number_from_first_and_last_digit(input, true), 32u32);
    }

    #[test]
    fn two_number_words_overlapping_in_str() {
        let input = "twone";
        assert_eq!(number_from_first_and_last_digit(input, true), 21u32);
    }

    #[test]
    fn test_input_file_2() {
        let input_data = include_str!("testinput_2.txt");
        assert_eq!(calculate_sum_from_codes(input_data, true), 281u32);
    }
}