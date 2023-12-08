use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

pub fn process(input: &str) -> u64 {
    let mut finished = false;
    let mut moves = 0;
    let traverse = parse_input(input);
    let mut instructions = traverse.0.iter().cycle();
    let mut current = "AAA";
    dbg!(&traverse);
    while !finished {
        moves += 1;
        match instructions.next().unwrap() {
            'R' => {
                current = &traverse.1.get(current).unwrap().right;
            }
            'L' => {
                current = &traverse.1.get(current).unwrap().left;
            }
            _ => panic!("invalid instruction"),
        }
        if current == "ZZZ" {
            finished = true;
        }
    }

    moves
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, Node>) {
    let mut input = input.lines();
    let instructions: Vec<char> = input.next().unwrap().chars().collect();
    input.next();
    let mut map = HashMap::new();
    input.for_each(|line| {
        let mut split = line.split(" = ");
        let id = split.next().unwrap();
        let mut split = split.next().unwrap().split(", ");
        let left: String = split
            .next()
            .unwrap()
            .chars()
            .filter(|c| *c != '(')
            .collect();
        let right: String = split
            .next()
            .unwrap()
            .chars()
            .filter(|c| *c != ')')
            .collect();
        map.insert(id, Node { left, right });
    });
    (instructions, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let expected = 6;
        let result = process(input);
        assert_eq!(result, expected);
    }

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
