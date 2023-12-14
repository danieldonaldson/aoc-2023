use std::collections::HashMap;

pub fn process(input: &str) -> i64 {
    // break into 2d grid
    // find starting position
    let (grid, start) = parse_into_grid(input);
    // dbg!(&grid);
    // crawl through the map until we reach the start again
    // println!("Starting at {:?}", &start);
    let distance = crawl(grid, start);
    distance / 2
    // possible directions: L/R=- U/D=| corners=FJL7
    // return half, rounded up?
}

fn crawl(grid: HashMap<(i64, i64), char>, start: (i64, i64)) -> i64 {
    let mut pos = start;
    let mut dir = (0, 1); //start by going down
    let mut steps = 0;
    loop {
        // move in direction
        pos.0 += dir.0;
        pos.1 += dir.1;
        steps += 1;

        // check if we've reached the start again
        if pos == start {
            break;
        }

        let c = grid.get(&pos).unwrap();
        // println!("Moving from {:?} ({})", pos, c);
        dir = match c {
            '|' => {
                if dir.1 == -1 {
                    (0, -1)
                } else if dir.1 == 1 {
                    (0, 1)
                } else {
                    panic!("Invalid approach direction for |")
                }
            }
            '-' => {
                if dir.0 == -1 {
                    (-1, 0)
                } else if dir.0 == 1 {
                    (1, 0)
                } else {
                    panic!("Invalid approach direction for -")
                }
            }
            'F' => {
                if dir.0 == -1 {
                    (0, 1)
                } else if dir.1 == -1 {
                    (1, 0)
                } else {
                    panic!("Invalid approach direction for F")
                }
            }
            'J' => {
                if dir.0 == 1 {
                    (0, -1)
                } else if dir.1 == 1 {
                    (-1, 0)
                } else {
                    panic!("Invalid approach direction for J")
                }
            }
            'L' => {
                if dir.0 == -1 {
                    (0, -1)
                } else if dir.1 == 1 {
                    (1, 0)
                } else {
                    panic!("Invalid approach direction for L")
                }
            }
            '7' => {
                if dir.0 == 1 {
                    (0, 1)
                } else if dir.1 == -1 {
                    (-1, 0)
                } else {
                    panic!("Invalid approach direction for 7")
                }
            }
            _ => panic!("Invalid character or not a loop! : {}", c),
        }
    }
    steps
}

fn parse_into_grid(input: &str) -> (HashMap<(i64, i64), char>, (i64, i64)) {
    let mut grid = HashMap::<(i64, i64), char>::new();
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i64, y as i64), c);
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }
    (grid, start)
}

#[cfg(test)]
mod tests {
    use super::*;
    //     #[test]
    //     fn test_process2() {
    //         let input = "7-F7-
    // .FJ|7
    // SJLL7
    // |F--J
    // LJ.LJ";
    //         let expected = 8;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }

    //     #[test]
    //     fn test_process() {
    //         let input = ".....
    // .S-7.
    // .|.|.
    // .L-J.
    // .....";
    //         let expected = 4;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
