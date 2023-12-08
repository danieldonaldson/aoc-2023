#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl Hand {
    fn from_cards(cards: Vec<Card>) -> Hand {
        let hm = cards
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            });

        if hm.values().any(|&v| v == 5) {
            Hand::FiveOfAKind
        } else if hm.values().any(|&v| v == 4) {
            return Hand::FourOfAKind;
        } else if hm.values().any(|&v| v == 3) {
            if hm.values().any(|&v| v == 2) {
                return Hand::FullHouse;
            } else {
                return Hand::ThreeOfAKind;
            }
        } else if hm.values().any(|&v| v == 2) {
            let pairs = hm.values().filter(|v| **v == 2).count();
            if pairs == 2 {
                return Hand::TwoPairs;
            } else {
                return Hand::OnePair;
            }
        } else {
            return Hand::HighCard;
        }
    }
}

#[derive(Debug)]
struct Game {
    bid: u64,
    cards: Vec<Card>,
    hand: Hand,
}

impl Game {
    fn new(bid: u64, cards: &str) -> Game {
        let cards: Vec<Card> = cards.chars().map(Card::from_char).collect();
        Game {
            bid,
            cards: cards.clone(),
            hand: Hand::from_cards(cards),
        }
    }

    fn position_score(&self) -> u64 {
        dbg!(&self.cards);
        dbg!(self.cards.iter().enumerate().fold(0, |acc, (i, card)| {
            acc + (*card as u64) * 15u64.pow((self.cards.len() - i - 1) as u32)
        }))
    }

    fn sorting_score(&self) -> u64 {
        dbg!(self.hand as u64 * 15u64.pow(self.cards.len() as u32 + 1));
        dbg!((self.hand as u64 * 15u64.pow(self.cards.len() as u32 + 1)) + self.position_score())
    }
}

pub fn process(input: &str) -> u64 {
    let mut games = parse_input(input);
    // dbg!(&games);
    games.sort_by_cached_key(|game| game.sorting_score());
    // dbg!(&games);
    games
        .iter()
        .enumerate()
        .inspect(|(i, game)| println!("{}: {:?}", i, game))
        .map(|(i, game)| (i as u64 + 1) * game.bid)
        .sum()
}

fn parse_input(input: &str) -> Vec<Game> {
    // dbg!(input);
    input
        .lines()
        // .inspect(|line| println!("{}", line))
        .map(|line| {
            let mut split = line.split(' ');
            // println!("{}", split.nth(1).unwrap());
            let card = split.next().unwrap();
            let bid = split.next().unwrap().parse::<u64>().unwrap();
            Game::new(bid, card)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_process() {
    //         let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    //         let expected = 6440;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }

    //     #[test]
    //     fn test_process() {
    //         let input = "AK 1
    // 22 2";
    //         let expected = 6440;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }

    // #[test]
    // fn test_sorting_score() {
    //     let input = "22";
    //     let expected = 2;
    //     let game = Game::new(0, input);
    //     let result = game.sorting_score();
    //     assert_eq!(result, expected);
    // }
}
