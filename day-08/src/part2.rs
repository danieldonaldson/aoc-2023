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
    dbg!(&traverse);
    let map = traverse.1;
    let mut instructions = traverse.0.iter().cycle();
    let mut current: HashMap<usize, &str> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .enumerate()
        .collect();
    let len = &current.len();
    dbg!(&current);

    let mut lcm = HashMap::new();

    loop {
        moves += 1;
        match instructions.next().unwrap() {
            'R' => current.iter_mut().for_each(|(_i, c)| {
                *c = &map.get(c).unwrap().right;
            }),
            'L' => current.iter_mut().for_each(|(_i, c)| {
                *c = &map.get(c).unwrap().left;
            }),
            _ => panic!("invalid instruction"),
        }
        if current.iter().filter(|(_i, k)| k.ends_with('Z')).count() == *len {
            break;
        }
        if let Some((i)) = current.iter().position(|(_i, k)| k.ends_with('Z')) {
            lcm.entry(i).or_insert(moves);
            println!("reached a Z ending at move {}", moves);
            dbg!(&current);
        }
        if lcm.len() == *len {
            dbg!(&lcm);
            break;
        }

        // dbg!(&current);
    }

    lcm.values().fold(1, |acc, v| num::integer::lcm(acc, *v))
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX";
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
