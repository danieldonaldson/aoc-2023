use regex::Regex;

pub fn process(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
    input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut min_colours = (0, 0, 0);
            for capture in re.captures_iter(line) {
                let number: u32 = capture[1].parse().unwrap();
                let color = &capture[2];
                match color {
                    "red" => min_colours.0 = number.max(min_colours.0),
                    "green" => min_colours.1 = number.max(min_colours.1),
                    "blue" => min_colours.2 = number.max(min_colours.2),
                    _ => {}
                }
                // println!("Color: {}, Number: {}", color, number);
            }
            println!("min_colours: {:?}", min_colours);
            min_colours.0 * min_colours.1 * min_colours.2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_basic() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = 48;
        let result = process(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
