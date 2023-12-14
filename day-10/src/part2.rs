use std::collections::HashMap;

pub fn process(input: &str) -> i64 {
    // break into 2d grid
    // find starting position
    let (grid, start) = parse_into_grid(input);
    let width = &grid.keys().map(|(x, _)| x).max().unwrap();
    // dbg!(&grid);
    // crawl through the map until we reach the start again
    // println!("Starting at {:?}", &start);
    let crawled_grid = crawl(grid.clone(), start);
    let mut count = 0;
    for (point, _) in grid.iter() {
        if point_in_polygon(*point, &crawled_grid, **width) {
            // println!("Point {:?} is in the polygon", point);
            count += 1;
        }
    }
    count
}

fn point_in_polygon(point: (i64, i64), polygon: &HashMap<(i64, i64), char>, width: i64) -> bool {
    if polygon
        .iter()
        .any(|(p, &c)| p == &point && (c == '1' || c == '0'))
    {
        // println!("Point {:?} is a vertex", point);
        return false;
    }
    let mut i = point.0 + 1;
    let mut count_intersections = 0;

    while i <= width {
        if let Some(&c) = polygon.get(&(i, point.1)) {
            if c == '1' {
                count_intersections += 1;
                // println!("Point {:?} is an intersection with ({},{})", point, i, j);
            }
        }
        i += 1;
    }
    if count_intersections % 2 == 1 {
        return true;
    }
    false
}

fn crawl(grid: HashMap<(i64, i64), char>, start: (i64, i64)) -> HashMap<(i64, i64), char> {
    let mut pos = start;
    let mut dir = (0, 1); //start by going down
                          // let mut steps = 0;
    let mut crawled_grid = grid.clone();
    crawled_grid.insert(pos, '1'); //S counts as 1 because it is F
    loop {
        // move in direction
        pos.0 += dir.0;
        pos.1 += dir.1;
        // steps += 1;

        // check if we've reached the start again
        if pos == start {
            break;
        }

        let c = &grid.get(&pos).unwrap();
        let edges = match c {
            'F' | '7' => '1',
            '|' => '1',
            'J' | 'L' | '-' => '0',
            _ => panic!("Invalid char"),
        };
        crawled_grid.insert(pos, edges);
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
    crawled_grid
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

    #[test]
    fn test_process() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let expected = 8;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
