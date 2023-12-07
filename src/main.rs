use std::fs::read_to_string;

fn setup(filename: &str) {
    let input = read_to_string(filename).unwrap();
}

fn part_1(filename: &str) -> i32 {
    setup(filename);
    0
}

fn main() {
    assert_eq!(part_1("example.txt"), 0);
    println!("Part 1 Solution: {}", part_1("input.txt"));
    // assert_eq!(part_2("example.txt"), 0);
    // println!("Part 2 Solution: {}", part_1("input.txt"));
}
