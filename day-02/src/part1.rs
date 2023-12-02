use regex::Regex;

pub fn process(input: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
    input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut split_game_id = line.split(": ");
            let game_id = split_game_id
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            for capture in re.captures_iter(line) {
                match &capture[2] {
                    "red" => {
                        if capture[1].parse::<u32>().unwrap() > max_red {
                            return 0;
                        }
                    }
                    "green" => {
                        if capture[1].parse::<u32>().unwrap() > max_green {
                            return 0;
                        }
                    }
                    "blue" => {
                        if capture[1].parse::<u32>().unwrap() > max_blue {
                            return 0;
                        }
                    }
                    _ => {}
                }
                // let number: u32 = capture[1].parse().unwrap();
                // let color = &capture[2];
                // println!("Color: {}, Number: {}", color, number);
            }
            game_id
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 8;
        let result = process(input, 12, 13, 14);
        assert_eq!(result, expected);
    }
}
