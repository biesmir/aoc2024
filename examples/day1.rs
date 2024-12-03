use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day1/input.txt")),
                2 => println!("{}", part2("Inputs/Day1/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day1/input.txt"));
        println!("{}", part2("Inputs/Day1/input.txt"));
    }
}

fn part1(input_file: &str) -> usize {
    let mut first_numbers: Vec<i32> = Vec::new();
    let mut second_numbers: Vec<i32> = Vec::new();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(first), Ok(second)) = (numbers[0].parse(), numbers[1].parse()) {
                first_numbers.push(first);
                second_numbers.push(second);
            }
        }
    }

    first_numbers.sort();
    second_numbers.sort();

    //let mut distances: Vec<usize> = Vec::new();
    let mut distance: usize = 0;
    for (first, second) in first_numbers.iter().zip(second_numbers.iter()) {
        distance += <i32 as TryInto<usize>>::try_into((first - second).abs()).unwrap();
    }

    return distance;
}

fn part2(input_file: &str) -> usize {
    let mut first_numbers: Vec<usize> = Vec::new();
    let mut second_numbers: Vec<usize> = Vec::new();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(first), Ok(second)) = (numbers[0].parse(), numbers[1].parse()) {
                first_numbers.push(first);
                second_numbers.push(second);
            }
        }
    }

    let mut result = 0;
    for number in first_numbers {
        result += number * second_numbers.iter().filter(|&n| *n == number).count();
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("Inputs/Day1/example.txt"), 11);
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(part2("Inputs/Day1/example.txt"), 31);
    }
}
