pub fn process(input: &str) -> i64 {
    let lines = parse_input(input);
    // dbg!(&lines);
    lines
        .into_iter()
        .map(|x| find_pattern(x).unwrap())
        // .inspect(|x| println!("{:?}", x))
        .sum()
}

fn find_pattern(line: Vec<i64>) -> Option<i64> {
    let first = line.first().unwrap();
    if line.iter().all(|n| *n == 0) {
        None
    } else {
        let r = find_pattern(differentiate(&line));
        match r {
            Some(n) => Some(first - n),
            None => Some(*first),
        }
    }
}

fn differentiate(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = 2;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
