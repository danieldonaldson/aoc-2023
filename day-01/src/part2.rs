pub fn process(input: &str) -> u32 {
    input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut first = 0;
            let mut last = 0;
            for index in 0..line.len() {
                let reduced_line = &line[index..];
                //match pattern with digit or typed digit
                first = if reduced_line.starts_with("one") {
                    1
                } else if reduced_line.starts_with("two") {
                    2
                } else if reduced_line.starts_with("three") {
                    3
                } else if reduced_line.starts_with("four") {
                    4
                } else if reduced_line.starts_with("five") {
                    5
                } else if reduced_line.starts_with("six") {
                    6
                } else if reduced_line.starts_with("seven") {
                    7
                } else if reduced_line.starts_with("eight") {
                    8
                } else if reduced_line.starts_with("nine") {
                    9
                } else if reduced_line.chars().next().unwrap().is_digit(10) {
                    reduced_line
                        .chars()
                        .next()
                        .unwrap()
                        .to_digit(10)
                        .expect("should be a digit")
                } else {
                    continue;
                };
                break;
            }

            for index in (0..line.len()).rev() {
                let reduced_line = &line[..index + 1];
                //find last instance of digit or typed digit
                // println!("reduced_line: {}", reduced_line);
                last = if reduced_line.ends_with("one") {
                    1
                } else if reduced_line.ends_with("two") {
                    2
                } else if reduced_line.ends_with("three") {
                    3
                } else if reduced_line.ends_with("four") {
                    4
                } else if reduced_line.ends_with("five") {
                    5
                } else if reduced_line.ends_with("six") {
                    6
                } else if reduced_line.ends_with("seven") {
                    7
                } else if reduced_line.ends_with("eight") {
                    8
                } else if reduced_line.ends_with("nine") {
                    9
                } else if reduced_line.chars().next_back().unwrap().is_digit(10) {
                    reduced_line
                        .chars()
                        .next_back()
                        .unwrap()
                        .to_digit(10)
                        .expect("should be a digit")
                } else {
                    continue;
                };
                break;
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected = 281;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
