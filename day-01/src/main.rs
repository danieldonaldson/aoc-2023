mod part1;
mod part2;

fn main() {
    let file = include_str!("../input.txt");
    let result = part1::process(file);
    println!("result for part 1: {}", result);

    let result = part2::process(file);
    println!("result for part 2: {}", result);
}
