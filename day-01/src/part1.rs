pub fn process(input: &str) -> u32 {
    input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let first = line
                .chars()
                // .inspect(|c| println!("char: {}", c))
                .find_map(|c| c.to_digit(10))
                .expect("should be a digit");
            let last_found = line.chars().rfind(|c| c.is_digit(10));
            let last: u32;
            if let Some(choice) = last_found {
                last = choice.to_digit(10).expect("should be a digit");
            } else {
                last = first;
            }
            // println!("first: {}, last: {}", first, last);
            last + first * 10
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = 142;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
