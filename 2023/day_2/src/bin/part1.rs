pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("example.txt");

fn main() {}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut split_line = line.split(": ");
            let game = split_line.next()?.split(' ').nth(1)?.parse::<u32>().ok()?;
            let rounds = split_line.next()?.split("; ");
            let valid = rounds
                .flat_map(|round| round.split(", "))
                .map(|show| {
                    let mut cube_type = show.split(' ');
                    let amount = cube_type.next()?.parse::<u32>().ok()?;
                    let color_str = cube_type.next()?;
                    match color_str {
                        "red" => Some(amount <= 12),
                        "green" => Some(amount <= 13),
                        "blue" => Some(amount <= 14),
                        _ => None,
                    }
                })
                .all(|x| x.unwrap_or(false));
            if valid {
                Some(game)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_works() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn input_works() {
        let result = part1(REAL_INPUT);
        assert_eq!(result, 2085);
    }
}
