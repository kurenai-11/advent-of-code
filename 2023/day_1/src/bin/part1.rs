fn main() {}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|v| v.to_digit(10)))
        .map(|mut r| {
            let first = r.next();
            let last = r.nth_back(0);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_works() {
        let example = include_str!("example.txt");
        let result = part1(example);
        assert_eq!(result, 142)
    }

    #[test]
    fn input_works() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 53921);
    }
}
