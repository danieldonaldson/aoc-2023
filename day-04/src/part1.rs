use std::collections::HashSet;

pub fn process(input: &str) -> u32 {
    input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut splitting = line.split(": ");
            let _ = splitting.next().unwrap(); //game ID
            let mut splitting = splitting.next().unwrap().split(" | ");
            let winning_numbers = splitting
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| x.parse::<u32>().is_ok());
            let your_numbers = splitting
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| x.parse::<u32>().is_ok());
            let mut winning_set = HashSet::<u32>::new();
            for number in winning_numbers {
                winning_set.insert(number.parse().unwrap());
            }
            let mut score = 0;
            // dbg!(&winning_set);
            for number in your_numbers {
                // dbg!(&number);
                if winning_set.contains(&number.parse::<u32>().unwrap()) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            score
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_process() {
    //         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    //         let expected = 13;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
