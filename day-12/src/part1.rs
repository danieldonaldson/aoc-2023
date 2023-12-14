#[derive(Debug)]
enum Record {
    Broken,
    Operational,
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
    dbg!(&grid);

    // let results: Vec<_> = grid.iter().filter(|a| is_valid_arrangement(a)).collect();
    // dbg!(results);

    //brute force every ? to be broken or operational
    for arrangement in grid.iter() {
        for record in arrangement.records.iter() {
            match record {
                Record::Unknown => {
                    //brute force every other unknown
                    todo!()
                },
                _ => continue,
            }
        }

    todo!()
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
    println!(
        "broken_group_counts: {:?}, arrangement.arrangement: {:?}",
        broken_group_counts, arrangement.arrangement
    );
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
    fn test_process2() {
        let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        let expected = 374;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
