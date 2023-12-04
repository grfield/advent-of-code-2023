fn main() {
    let input_data = include_str!("input_data.txt");
    println!("Day 1a answer: {}", calculate_sum_from_codes(input_data));
}

fn calculate_sum_from_codes(input: &str) -> u32 {
    input.lines()
        .map(|l| { number_from_first_and_last_digit(l)})
        .sum::<u32>()
}

fn find_digits_or_words(line: &str) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();

    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            digits.push(ch);
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
    fn only_two_digits_in_str() {
        let input = "bb5fdwDvj6V";
        assert_eq!(number_from_first_and_last_digit(input), 56u32);
    }

    #[test]
    fn only_one_digit_in_str() {
        let input = "z1dwDvjBgV";
        assert_eq!(number_from_first_and_last_digit(input), 11u32);
    }

    #[test]
    fn test_input_file_1() {
        let input_data = include_str!("testinput.txt");
        assert_eq!(calculate_sum_from_codes(input_data), 142u32);
    }
}