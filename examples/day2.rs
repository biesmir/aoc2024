use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_matrix(input_file: &str) -> Vec<Vec<isize>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        matrix.push(row);
    }
    matrix
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day2/input.txt")),
                2 => println!("{}", part2("Inputs/Day2/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day2/input.txt"));
        println!("{}", part2("Inputs/Day2/input.txt"));
    }
}

fn is_row_safe(row: Vec<isize>) -> bool {
    let mut rising: Option<bool> = None;
    for window in row.windows(2) {
        let diff = window[0] - window[1];
        if (diff) > 0 {
            if rising == Some(false) {
                return false;
            } else {
                rising = Some(true);
            }
            if diff > 3 {
                return false;
            }
        } else if (diff) < 0 {
            if rising == Some(true) {
                return false;
            } else {
                rising = Some(false);
            }
            if diff < -3 {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn part1(input: &str) -> usize {
    let matrix = read_matrix(input);
    let mut safe = 0;
    for line in matrix {
        if is_row_safe(line) {
            safe += 1;
        }
    }
    return safe;
}

fn part2(input: &str) -> usize {
    let matrix = read_matrix(input);
    let mut safe = 0;
    for line in matrix {
        for i in 0..line.len() {
            let mut line_alter = line.clone();
            line_alter.remove(i);
            if is_row_safe(line_alter) {
                safe += 1;
                break;
            }
        }
    }
    return safe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day2/example.txt"), 2);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day2/example.txt"), 4);
    }
}
