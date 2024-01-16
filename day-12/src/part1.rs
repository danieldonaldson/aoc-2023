#[derive(Debug)]
enum Record {
    Broken = 0,
    Operational = 1,
    Unknown,
}

#[derive(Debug)]
struct Arrangements {
    records: Vec<Record>,
    arrangement: Vec<usize>,
}

pub fn process(input: &str) -> i64 {
    // break into 2d grid
    // find starting position
    let grid = parse_input(input);
    // dbg!(&grid);

    // let results: Vec<_> = grid.iter().filter(|a| is_valid_arrangement(a)).collect();
    // dbg!(results);

    grid.iter()
        .map(|arrangement| {
            let mut counter = 0;
            // println!("arrangement: {:?}",&arrangement);
            let length = generate_unknown_combinations(&arrangement);
            for i in 0..length {
                let new_records =
                    iterate_unknown_combination(&arrangement.records, i, length.ilog2() as usize);
                let new_arrangement = Arrangements {
                    records: new_records,
                    arrangement: arrangement.arrangement.clone(),
                };
                if is_valid_arrangement(&new_arrangement) {
                    // dbg!(new_arrangement);
                    counter += 1;
                }
            }
            counter
        })
        .sum()
}

fn generate_unknown_combinations(arrangement: &Arrangements) -> usize {
    2usize.pow(
        arrangement
            .records
            .iter()
            .filter(|record| match record {
                Record::Unknown => true,
                _ => false,
            })
            .count() as u32,
    )
}

fn iterate_unknown_combination(records: &Vec<Record>, counter: usize, width: usize) -> Vec<Record> {
    let binary_str = format!("{:0width$b}", counter, width = width);
    let mut binary_rep = binary_str.chars();
    // println!("{}", binary_str);
    let mut new_records = Vec::<Record>::new();
    for record in records.iter() {
        match record {
            Record::Broken => new_records.push(Record::Broken),
            Record::Operational => new_records.push(Record::Operational),
            Record::Unknown => match binary_rep.next().unwrap() {
                '0' => new_records.push(Record::Broken),
                '1' => new_records.push(Record::Operational),
                _ => panic!("Invalid binary rep"),
            },
        }
    }
    new_records
}

fn is_valid_arrangement(arrangement: &Arrangements) -> bool {
    // find groups of broken records
    let mut broken_groups = vec![];
    let mut current_group = vec![];
    for record in arrangement.records.iter() {
        match record {
            Record::Broken => {
                current_group.push(record);
            }
            Record::Operational => {
                broken_groups.push(current_group.clone());
                current_group.clear();
            }
            Record::Unknown => panic!("Unknown record"),
        }
    }
    if !current_group.is_empty() {
        broken_groups.push(current_group.clone());
    }
    // dbg!(&broken_groups);
    //count records in broken groups
    let broken_group_counts: Vec<_> = broken_groups
        .iter()
        .map(|g| g.len())
        .filter(|group| *group != 0)
        .collect();
    //compare to arrangement
    // println!(
    //     "broken_group_counts: {:?}, arrangement.arrangement: {:?}",
    //     broken_group_counts, arrangement.arrangement
    // );
    broken_group_counts == arrangement.arrangement
}

fn parse_input(input: &str) -> Vec<Arrangements> {
    input
        .lines()
        // .inspect(|f| println!("{}", f))
        .map(|line| {
            let mut split = line.split(' ');
            let records: Vec<Record> = split
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Record::Operational,
                    '#' => Record::Broken,
                    '?' => Record::Unknown,
                    _ => panic!("Invalid input"),
                })
                .collect();

            let arrangement: Vec<usize> = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            Arrangements {
                records,
                arrangement,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let expected = 21;
        let result = process(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_combinations() {
        let records = vec![Record::Unknown, Record::Unknown, Record::Unknown];
        let arrangement = vec![1];
        let input = Arrangements {
            records,
            arrangement,
        };
        let expected = 8;
        let result = generate_unknown_combinations(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_binary_reps() {
        let records = vec![Record::Unknown, Record::Unknown, Record::Unknown];
        let arrangement = vec![1];
        let input = Arrangements {
            records,
            arrangement,
        };
        let expected = 8;
        let result = iterate_unknown_combination(&input.records, 8, 3);
        assert_eq!(false, true);
    }
}
