use regex::Regex;

pub fn process(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)").unwrap();
    let lines: Vec<&str> = input
        .lines()
        // .inspect(|line| println!("line: {}", line))
        .collect();
    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        // println!("line: {}", line);
        for found in re.captures_iter(line) {
            let number: u32 = found[1].parse().unwrap();
            // println!("Number: {}", number);

            let adjacent = if i == 0 {
                ["", lines[i], lines[i + 1]]
            } else if i == lines.len() - 1 {
                [lines[i - 1], lines[i], ""]
            } else {
                [lines[i - 1], lines[i], lines[i + 1]]
            };
            // println!("adjacent: {:?}", adjacent);
            if has_symbol_adjacent(number, found.get(0).unwrap().start(), &adjacent) {
                sum += number;
                println!("Number matched!: {}", number);
            }
        }
    }
    sum

    // .sum::<u32>()
}

fn has_symbol_adjacent(num: u32, position_start: usize, adjacent: &[&str; 3]) -> bool {
    let num_length = num.to_string().len();
    let symbol_regex = Regex::new(r"[*@=+/\-#%%#$&]").unwrap();
    // println!(
    //     "num: {}, position_start: {}, adjacent: {:?}",
    //     num, position_start, adjacent
    // );
    //adjacent:
    // ..aaa.. [position: Y-1; between X-1 and X+1]
    // ..aXa.. [position: Y; X-1 or X+1]
    // ..aaa.. [position: Y+1; between X-1 and X+1]
    let symbols = symbol_regex.find_iter(adjacent[1]);

    for symbol in symbols {
        // println!("symbols adjacent line: {:?}", symbol);
        if position_start > 0 && symbol.start() == position_start - 1 {
            return true;
        }
        if symbol.start() == position_start + num_length {
            return true;
        }
    }

    let symbols = symbol_regex.find_iter(adjacent[0]);

    for symbol in symbols {
        // println!("symbols line[0]: {:?}", symbol);
        let symbol_match_start = if position_start == 0 {
            0
        } else {
            symbol.start() + 1
        };
        if symbol_match_start >= position_start && symbol.start() <= position_start + num_length {
            return true;
        }
    }

    let symbols = symbol_regex.find_iter(adjacent[2]);

    for symbol in symbols {
        // println!("symbols line[2]: {:?}", symbol);
        let symbol_match_start = if position_start == 0 {
            0
        } else {
            symbol.start() + 1
        };
        if symbol_match_start >= position_start && symbol.start() <= position_start + num_length {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_adjacent() {
    //     let input = ["..........", ".535...44.", ".........."];
    //     let expected = true;
    //     let result = has_symbol_adjacent(535, 1, &input);
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
    //         let expected = 4361;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
