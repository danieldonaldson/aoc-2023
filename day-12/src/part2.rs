use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Arrangements<'a> {
    records: String,
    arrangement: Vec<usize>,
    memo: HashMap<&'a str, Vec<usize>>,
    matches: usize,
}

impl Arrangements<'_> {
    fn check_string(mut self: Self, str: &str, groups: Vec<usize>) {
        if str.is_empty() && groups.is_empty() {
            self.matches += 1;
            return;
        }
        let first = str.chars().nth(0);
        println!("checking string: {:?} for {:?}", str, groups);

        match first {
            Some('.') => {
                self.check_string(&str[1..], groups);
            },
            Some('?') => {
                self.check_string(&[".", &str[1..]].join(""), groups.clone());
                &self.check_string(&["#", &str[1..]].join(""), groups);
            },
            Some('#') => todo!(),
            _ => panic!("Invalid input")
        }





        if first == Some('.') {
            Self::check_string(self, &str[1..], groups);
        } else if first == Some('?') {
            Self::check_string(self.clone(), &[".", &str[1..]].join(""), groups.clone());
            Self::check_string(self, &["#", &str[1..]].join(""), groups);
        } else if first == Some('#') {
            let group_length = groups[0];
            if str.len() >= group_length {
                let mut valid = true;
                let mut length = 0;
                for c in str.chars() {
                    if c == '#' {
                        length += 1;
                    } else if c == '?' {
                        Self::check_string(self.clone(), &["#", &str[1..]].join(""), groups.clone());
                        Self::check_string(self.clone(), &[".", &str[1..]].join(""), groups.clone());
                    }
                    else {
                        break;
                    }
                }
                if length != group_length {
                    valid = false;
                }
                if valid {
                    println!("valid group found for {}: {:?}", str, groups);
                    if groups.len() == 1 {
                        // valid
                        self.matches += 1;
                    } else {
                        Self::check_string(self, &str[group_length..], groups[1..].to_vec());
                    }
                } else if groups.len() == 1 {
                    // invalid
                }
                else {
                    Self::check_string(self, &str[1..], groups.clone());
                }
            }
        }
    }
}

pub fn process(input: &str) -> usize {
    let grid = parse_input(input);

    grid.par_iter()
        .map(|arrangement| {
            let arr1 = arrangement.clone();
            arrangement
                .clone()
                .check_string(arr1.records.as_str(), arr1.arrangement);
            println!("arrangements for {:?}", arrangement);
            arrangement.matches
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Arrangements> {
    input
        .lines()
        // .inspect(|f| println!("{}", f))
        .map(|line| {
            let mut split = line.split(' ');
            let mut records = split.next().unwrap().to_owned();
            records.push('.');

            let arrangement: Vec<usize> = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let mut five_x_records = records.clone().to_owned();
            // for i in 0..4 {
            //     five_x_records = [five_x_records, records.clone().to_owned()].join("?");
            // }

            let mut five_x_arrangements = arrangement.clone();
            // for i in 0..4 {
            //     five_x_arrangements.append(&mut arrangement.clone());
            // }

            Arrangements {
                records: five_x_records,
                arrangement: five_x_arrangements,
                memo: HashMap::new(),
                matches: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "???.### 1,1,3";
        let expected = 525152;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
