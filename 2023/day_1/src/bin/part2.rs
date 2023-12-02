pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE2_INPUT: &str = include_str!("example_2.txt");

fn main() {
    let example2_result = part2(EXAMPLE2_INPUT);
    let result2 = part2(REAL_INPUT);
    println!("example2 result: {}", example2_result);
    println!("result2: {result2}");
}

#[derive(Debug)]
struct Word<'a> {
    string: &'a str,
    numeric: u32,
}

// dirty approach, my first time (and using rust) lol sorry
pub fn part2(input: &str) -> u32 {
    let words: Vec<Word> = (1..=9)
        .zip([
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ])
        .map(|(num, word)| Word {
            string: word,
            numeric: num,
        })
        .collect();
    input
        .lines()
        .map(|line| {
            let mut matches = vec![];
            line.chars().fold("".to_owned(), |mut acc, cur_char| {
                acc.push(cur_char);
                if let Some(contained_word) = words.iter().find(|word| acc.ends_with(word.string)) {
                    matches.push(contained_word.numeric);
                } else if let Some(numeric_val) = cur_char.to_digit(10) {
                    matches.push(numeric_val)
                }
                acc
            });
            matches
        })
        .map(|matches| {
            let first = matches.first();
            let last = matches.last();
            if let (Some(first), Some(last)) = (first, last) {
                format!("{}{}", first, last)
            } else if let (Some(first), None) = (first, last) {
                format!("{}{}", first, first)
            } else {
                panic!("the string has no numbers at all")
            }
        })
        .map(|p| p.parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p2_example_works() {
        let result = part2(EXAMPLE2_INPUT);
        assert_eq!(result, 281);
    }

    #[test]
    fn p2_input_works() {
        let result = part2(REAL_INPUT);
        assert_eq!(result, 54676)
    }
}
