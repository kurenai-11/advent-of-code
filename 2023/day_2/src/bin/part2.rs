pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("example.txt");

fn main() {}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|rounds| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for show in rounds.split("; ").flat_map(|round| round.split(", ")) {
                let mut cube_type = show.split(' ');
                let amount = cube_type.next().unwrap().parse::<u32>().unwrap();
                match cube_type.next().unwrap() {
                    "red" => r = r.max(amount),
                    "green" => g = g.max(amount),
                    "blue" => b = b.max(amount),
                    color => panic!("unknown color, {}", color),
                };
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
