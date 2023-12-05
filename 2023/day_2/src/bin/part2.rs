pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("example.txt");

fn main() {}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|rounds| {
            let split_rounds: Vec<&str> = rounds.split("; ").collect();
            let (mut r, mut g, mut b) = (0, 0, 0);
            for round in split_rounds {
                let shows = round.split(", ").collect::<Vec<&str>>();
                for show in shows {
                    let cube_type = show.split(' ').collect::<Vec<&str>>();
                    let (amount, color_str) = (cube_type[0].parse::<u32>().unwrap(), cube_type[1]);
                    match color_str {
                        "red" => r = r.max(amount),
                        "green" => g = g.max(amount),
                        "blue" => b = b.max(amount),
                        _ => panic!("unknown color, {}", color_str),
                    };
                }
            }
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_works() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 2286);
    }

    #[test]
    fn input_works() {
        let result = part2(REAL_INPUT);
        assert_eq!(result, 79315);
    }
}
