pub const REAL_INPUT: &str = include_str!("input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("example.txt");

fn main() {}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(':').map(|p| p.trim()).collect::<Vec<&str>>())
        .map(|split_line| {
            let game = split_line[0]
                .split(' ')
                .map(|p| p.trim())
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let rounds: Vec<&str> = split_line[1].split(';').map(|p| p.trim()).collect();
            let mut valid = true;
            for round in rounds {
                if !valid {
                    break;
                }
                let shows = round.split(',').map(|p| p.trim()).collect::<Vec<&str>>();
                for show in shows {
                    if !valid {
                        break;
                    }
                    let cube_type = show.split(' ').collect::<Vec<&str>>();
                    let (amount, color_str) = (cube_type[0].parse::<u32>().unwrap(), cube_type[1]);
                    valid = match color_str {
                        "red" => amount <= 12,
                        "green" => amount <= 13,
                        "blue" => amount <= 14,
                        _ => panic!("unknown color, {}", color_str),
                    };
                }
            }
            if valid {
                game
            } else {
                0
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
        assert_eq!(result, 8);
    }
}
