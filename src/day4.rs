use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");
        assert_eq!(run_part1(&lines), 18_i128);

        let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
        assert!(run_part1(&lines) < 2622_i128);
        // 2622 too high
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");

        assert_eq!(run_part2(&lines), 9_i128);
    }
}

pub fn searcher(input: &Vec<String>, check_for: &String, start: (usize, usize), dir: (i32, i32)) -> usize {
    let mut r = start.0 as i32;
    let mut c = start.1 as i32;
    for ch in check_for.chars(){
        match  input.get(r as usize) {
            Some(li) => {
                match li.chars().nth(c as usize){
                    Some(cha) => {
                        if ch != cha {
                            return 0;
                        }
                        r += dir.0;
                        c += dir.1;
                    }
                    None => {
                        return 0;
                    }
                }

            }
            None => {
                return 0;
            }
        }

    }
    1
}
pub fn check(input: &Vec<String>, check_for: String) -> usize {
    let mut ret = 0;

    let width = input[0].len() ;
    let height =  input.len();

    let check_str = check_for.chars().next();
    if check_str.is_none() { return 1; }
    let check_char = check_str.unwrap();
    let check_len = check_for.len() - 1;
    for (row, line) in input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == check_char {
                let check_down = row <= height- check_len;
                let check_up = row >= check_len;
                let check_right = col <= width- check_len;
                let check_left = col >= check_len;
                let checker = &check_for[1..].to_string();
                if check_right {
                    ret+=searcher(input, checker, (row, col+1), (0,1));
                }
                if check_left {
                    ret+=searcher(input, checker, (row, col-1), (0,-1));
                }
                if check_down {
                    ret+=searcher(input, checker, (row+1, col), (1,0));
                }
                if check_up {
                    ret+=searcher(input, checker, (row-1, col), (-1,0));
                }
                if check_up && check_left {
                    ret+=searcher(input, checker, (row-1, col-1), (-1,-1));
                }
                if check_up && check_right {
                    ret+=searcher(input, checker, (row-1, col+1), (-1,1));
                }
                if check_down && check_left {
                    ret+=searcher(input, checker, (row+1, col-1), (1,-1));
                }
                if check_down && check_right {
                    ret+=searcher(input, checker, (row+1, col+1), (1,1));
                }
            }
        }
    }
    ret
}
pub fn check2(input: &Vec<String>) -> usize {
    let mut ret = 0;
    let cols =  input[0].len();
    let rows = input.len();
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            // Search for A
            match input.get(row).unwrap().chars().nth(col){
                Some(ch) => {
                    if ch == 'A' {
                        let ul = input.get(row-1).unwrap().chars().nth(col-1).unwrap();
                        let ur = input.get(row-1).unwrap().chars().nth(col+1).unwrap();
                        let dl = input.get(row+1).unwrap().chars().nth(col-1).unwrap();
                        let dr = input.get(row+1).unwrap().chars().nth(col+1).unwrap();

                        if         ul == 'M' && dr == 'S' && ur == 'M' && dl == 'S' {
                            ret += 1;
                        } else if  ul == 'S' && dr == 'M' && ur == 'S' && dl == 'M' {
                            ret += 1;
                        } else if  ul == 'M' && dr == 'S' && ur == 'S' && dl == 'M' {
                            ret += 1;
                        } else if  ul == 'S' && dr == 'M' && ur == 'M' && dl == 'S' {
                            ret += 1;
                        }

                    }
                }
                None => {}
            }

        }
    }


    ret
}
pub fn run_part1(input: &Vec<String>) -> i128 {
    let res =check(input, "XMAS".to_string());
    res as i128
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    check2(input) as i128
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 4");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
