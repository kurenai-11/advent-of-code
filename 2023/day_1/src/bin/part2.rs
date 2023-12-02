pub const EXAMPLE1_INPUT: &str = include_str!("example.txt");
pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE2_INPUT: &str = include_str!("example_2.txt");

fn main() {
    let example1_result = part1(EXAMPLE1_INPUT);
    let example2_result = part2(EXAMPLE2_INPUT);
    let result1 = part1(REAL_INPUT);
    let result2 = part2(REAL_INPUT);
    println!("example1 result: {}", example1_result);
    println!("example2 result: {}", example2_result);
    println!("result1: {result1}");
    println!("result2: {result2}");
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|v| v.to_digit(10)))
        .map(|mut r| {
            let first = r.next();
            let last = r.last();
            if let (Some(val1), Some(val2)) = (first, last) {
                format!("{}{}", val1, val2)
            } else if let (Some(val1), None) = (first, last) {
                format!("{}{}", val1, val1)
            } else {
                panic!("the string has no numbers at all")
            }
        })
        .map(|p| p.parse::<u32>().unwrap())
        .sum()
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
            line.chars().fold("".to_owned(), |acc, cur_char| {
                let new_str = acc + cur_char.to_string().as_str();
                if let Some(contained_word) =
                    words.iter().find(|word| new_str.ends_with(word.string))
                {
                    matches.push(contained_word.numeric);
                } else if let Some(numeric_val) = cur_char.to_digit(10) {
                    matches.push(numeric_val)
                }
                new_str
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
    fn p1_example_works() {
        let result = part1(EXAMPLE1_INPUT);
        assert_eq!(result, 142)
    }

    #[test]
    fn p1_input_works() {
        let result = part1(REAL_INPUT);
        assert_eq!(result, 53921);
    }

    #[test]
    fn p2_example_works() {
        let result = part2(EXAMPLE2_INPUT);
        assert_eq!(result, 281);
    }

    #[test]
    fn p2_input_works() {
        let result = part2(REAL_INPUT);
        assert_eq!(result, 11)
    }
}
