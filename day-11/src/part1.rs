use std::collections::HashMap;

pub fn process(input: &str) -> i64 {
    // break into 2d grid
    // find starting position
    let grid = parse_into_grid(input);
    dbg!(&grid);

    //find distance to every point
    let results: Vec<_> = grid
        .values()
        .enumerate()
        .flat_map(|(i, &point1)| {
            grid.values()
                .skip(i + 1)
                .map(move |&point2| (point1, point2))
        })
        .map(|(point1, point2)| distance_between_points(point1, point2))
        .collect();

    results.iter().sum()
}

fn distance_between_points(a: (i64, i64), b: (i64, i64)) -> i64 {
    let x = (a.0 - b.0).abs();
    let y = (a.1 - b.1).abs();
    x + y
}

fn parse_into_grid(input: &str) -> HashMap<i64, (i64, i64)> {
    //assign # to numerical value
    let mut grid = HashMap::new();
    let mut counter = 1;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                grid.insert(counter, (x as i64, y as i64));
                counter += 1;
            }
        })
    });

    dbg!(grid.clone());

    //expand empty rows
    let max_x = grid.values().map(|(x, _)| x).max().unwrap();

    for i in (0..=*max_x).rev() {
        // if no point exists at this x value
        let exists = grid.values().any(|(x, _)| x == &i);
        if !exists {
            println!("No point at x={}", i);
            // add 1 to every following x value
            grid.iter_mut().for_each(|(_, (x, _))| {
                if *x > i {
                    *x += 1;
                }
            });
        }
    }

    let max_y = grid.values().map(|(_, y)| y).max().unwrap();
    for i in (0..=*max_y).rev() {
        // if no point exists at this y value
        if !&grid.values().any(|(_, y)| y == &i) {
            println!("No point at y={}", i);
            // add 1 to every following y value
            grid.iter_mut().for_each(|(_, (_, y))| {
                if *y > i {
                    *y += 1;
                }
            });
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    //     #[test]
    //     fn test_process2() {
    //         let input = "...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#.....";
    //         let expected = 374;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
