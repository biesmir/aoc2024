use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(filename: &str) -> io::Result<Vec<(usize, Vec<usize>)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Create a vector to store our matrix
    let mut input: Vec<(usize, Vec<usize>)> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            panic!()
        }
        let sol = parts[0].trim().parse().ok().unwrap();
        let nums: Vec<usize> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
        input.push((sol, nums));
    }

    Ok(input)
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day7/input.txt")),
                2 => println!("{}", part2("Inputs/Day7/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day7/input.txt"));
        println!("{}", part2("Inputs/Day7/input.txt"));
    }
}

fn is_correct(sol: usize, nums: &Vec<usize>, permutation: usize) -> bool {
    let mut res: usize = nums[0];
    // println!("permutation {}", permutation);
    for i in 0..nums.len() - 1 {
        if (permutation & (0x1 << i)) > 0 {
            res *= nums[i + 1]
        } else {
            res += nums[i + 1]
        }
    }
    if sol == res {
        return true;
    }
    // println!("{} != {}", sol, res);
    return false;
}

fn concat(num1: usize, num2:usize)->usize{
    for i in 0..20{
        if num2 % 10usize.pow(i) == num2{
            return 10usize.pow(i)*num1 + num2;
        }
    }
    panic!();
}

fn is_correct_with_concat(sol: usize, nums: &Vec<usize>, permutation: usize) -> bool {
    let mut res: usize = nums[0];
    // println!("permutation {}", permutation);
    for i in 0..nums.len() - 1 {
        let digit =   (permutation / 3usize.pow(i as u32)) % 3;
        match digit {
            0=>res *= nums[i + 1],
            1=>res += nums[i + 1],
            2=>res = concat(res, nums[i + 1]),
            _=> panic!(),
        }

    }
    if sol == res {
        return true;
    }
    // println!("{} != {}", sol, res);
    return false;
}

fn is_correctable(sol: usize, nums: &Vec<usize>) -> bool {
    let base: usize = 2;
    for permutation in 0..(base.pow((nums.len() - 1).try_into().unwrap())) + 1 {
        if is_correct(sol, &nums, permutation){
            return true;
        }
    }

    return false;
}

fn is_correctable_with_concat(sol: usize, nums: &Vec<usize>) -> bool {
    let base: usize = 3;
    for permutation in 0..(base.pow((nums.len() - 1).try_into().unwrap())) + 1 {
        if is_correct_with_concat(sol, &nums, permutation){
            return true;
        }
    }

    return false;
}

fn part1(file_name: &str) -> usize {
    let input = read_file(file_name).unwrap();
    let mut res = 0;
    for line in input{
        if is_correctable(line.0, &line.1){
            res += line.0;
        }
    }
    return res;
}

fn part2(file_name: &str) -> usize {
    let input = read_file(file_name).unwrap();
    let mut res = 0;
    for line in input{
        if is_correctable_with_concat(line.0, &line.1){
            res += line.0;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day7/example.txt"), 3749);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day7/example.txt"), 11387);
    }
}
