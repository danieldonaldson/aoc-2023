use regex::Regex;

pub fn process(input: &str) -> u32 {
    let re = Regex::new(r"(\*)").unwrap();
    let lines: Vec<&str> = input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .collect();
    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        // println!("line: {}", line);
        for found in re.captures_iter(line) {
            let adjacent = if i == 0 {
                ["", lines[i], lines[i + 1]]
            } else if i == lines.len() - 1 {
                [lines[i - 1], lines[i], ""]
            } else {
                [lines[i - 1], lines[i], lines[i + 1]]
            };
            // println!("adjacent: {:?}", adjacent);
            let number = has_gears_adjacent(found.get(0).unwrap().start(), &adjacent);
            sum += number;
            println!("Number matched!: {}", number);
        }
    }
    sum

    // .sum::<u32>()
}

fn has_gears_adjacent(position_star: usize, adjacent: &[&str; 3]) -> u32 {
    let symbol_regex = Regex::new(r"(\d+)").unwrap();
    let mut gears_matched: Vec<u32> = vec![];
    println!("position_star: {}, adjacent: {:?}", position_star, adjacent);
    //adjacent:
    // ..abc.. [position: Y-1; between X-1 and X+1]
    // ..c*d.. [position: Y; X-1 or X+1]
    // ..efg.. [position: Y+1; between X-1 and X+1]

    let symbols = symbol_regex.find_iter(adjacent[1]);
    for symbol in symbols {
        println!("symbols adjacent line: {:?}", symbol);
        if symbol.end() == position_star {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
        }
        if symbol.start() == position_star + 1 {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
        }
    }

    let symbols = symbol_regex.find_iter(adjacent[0]);
    for symbol in symbols {
        println!("symbols top line: {:?}", symbol);

        if symbol.end() >= position_star && symbol.end() - 1 <= position_star + 1 {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
            continue;
        }
        if symbol.start() >= position_star - 1 && symbol.start() <= position_star + 1 {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
        }
    }

    let symbols = symbol_regex.find_iter(adjacent[2]);
    for symbol in symbols {
        println!("symbols top line: {:?}", symbol);
        if symbol.end() >= position_star && symbol.end() - 1 <= position_star + 1 {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
            continue;
        }
        if symbol.start() >= position_star - 1 && symbol.start() <= position_star + 1 {
            println!("matched: {}", symbol.as_str());
            gears_matched.push(symbol.as_str().parse().unwrap());
        }
    }

    // let symbols = symbol_regex.find_iter(adjacent[2]);

    // for symbol in symbols {
    //     // println!("symbols line[2]: {:?}", symbol);
    //     let symbol_match_start = if position_start == 0 {
    //         0
    //     } else {
    //         symbol.start() + 1
    //     };
    //     if symbol_match_start >= position_start && symbol.start() <= position_start + num_length {
    //         return true;
    //     }
    // }
    if gears_matched.len() != 2 {
        return 0;
    }
    gears_matched.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_adjacent() {
    //     let input = ["10000.....", ".*.......", "1........."];
    //     let expected = 10000;
    //     let result = has_gears_adjacent(1, &input);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn test_process_1() {
    //     let input = "..35*..44.";
    //     let expected = 114;
    //     let result = process(input);
    //     assert_eq!(result, expected);
    // }
    //     #[test]
    //     fn test_process() {
    //         let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";
    //         let expected = 467835;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
