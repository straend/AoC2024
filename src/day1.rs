use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part1(&lines), 11_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part2(&lines), 31_i128);
    }
}
fn parse_input(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.iter() {
        let numbers = line
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        left.push(*numbers.first().unwrap());
        right.push(*numbers.last().unwrap());
    }
    left.sort();
    right.sort();

    (left, right)
}
pub fn run_part1(input: &Vec<String>) -> i128 {
    let (left, right) = parse_input(input);

    let mut diff: i32 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        diff += (l - r).abs();
    }
    diff as i128
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let (left, right) = parse_input(input);

    let mut sim = 0;
    for l in left.iter() {
        let c = (right.iter().filter(|x| *x == l).count()) as i32;
        sim += l * c;
    }

    sim as i128
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 1");
    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day1.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
