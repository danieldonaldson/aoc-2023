mod part1;

fn main() {
    let file = include_str!("../input.txt");
    let result = part1::process(file);
    println!("result for part 1: {}", result);
}
