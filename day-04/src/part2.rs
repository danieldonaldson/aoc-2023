use std::collections::{BTreeMap, HashSet};

pub fn process(input: &str) -> u32 {
    let mut cards_available = BTreeMap::<u32, u32>::new();
    input
        .lines()
        .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut splitting = line.split(": ");
            let game_id = splitting
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let cards_this_game = *(cards_available.get(&game_id).unwrap_or(&0)) + 1;

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
            // dbg!(&winning_set);

            let mut wins_this_game = 0;
            for number in your_numbers {
                // dbg!(&number);
                if winning_set.contains(&number.parse::<u32>().unwrap()) {
                    wins_this_game += 1;
                }
            }
            println!("wins_this_game: {}", wins_this_game);
            println!("cards_this_game: {}", cards_this_game);

            for i in 1..(wins_this_game + 1) {
                cards_available.insert(
                    game_id + i,
                    cards_available.get(&(game_id + i)).unwrap_or(&0) + cards_this_game,
                );
            }

            println!("cards_available: {:?}", cards_available);

            cards_this_game
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 30;
        let result = process(input);
        assert_eq!(result, expected);
    }
    // #[test]
    // fn test_process_1() {
    //     // 1 instance of card 1
    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    //     let expected = 1;
    //     let result = process(input);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn test_process_2() {
    //     // 1 instance of card 1 and 2 x card 2
    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    //     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
    //     let expected = 3;
    //     let result = process(input);
    //     assert_eq!(result, expected);
    // }

    //     #[test]
    //     fn test_process_3() {
    //         // 1 instance of card 1, 2 x card 2, 4 x card 3
    //         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    //         let expected = 7;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
