use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_to_vec(filename: &str) -> io::Result<Vec<usize>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        return Ok(line?
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect());
    }
    panic!()
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day11/input.txt")),
                2 => println!("{}", part2("Inputs/Day11/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day11/input.txt"));
        println!("{}", part2("Inputs/Day11/input.txt"));
    }
}

fn split_digits(num: usize, count: u32) -> (usize, usize) {
    let second = num % usize::pow(10, count / 2);
    let first = num / usize::pow(10, count / 2);
    return (first, second);
}
fn count_digits(num: usize) -> u32 {
    return (num.ilog10()) + 1;
}

// fn blink(stones: &mut Vec<usize>) {
//     let mut len = stones.len();

//     let mut i = 0;
//     while i < len {
//         if stones[i] == 0 {
//             stones[i] = 1;
//         } else {
//             let digits = count_digits(stones[i]);
//             if digits % 2 == 0 {
//                 let (first, second) = split_digits(stones[i], digits);
//                 stones[i] = first;
//                 stones.insert(i + 1, second);
//                 len += 1;
//                 i += 1;
//             } else {
//                 stones[i] *= 2024;
//             }
//         }
//         i += 1;
//     }
// }


fn part1(file_name: &str) -> usize {
    let mut stones: HashSet<usize> =
        HashSet::from_iter(read_file_to_vec(file_name).unwrap().iter().cloned());
    let mut nums: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

    for _ in 0..74 {
        let mut to_check: HashSet<usize> = HashSet::new();

        for stone in &stones {
            if *stone == 0 {
                nums.insert(*stone, (Some(1), None));
                to_check.insert(1);
            } else {
                let digits = count_digits(*stone);
                if digits % 2 == 0 {
                    let (first, second) = split_digits(*stone, digits);
                    to_check.insert(first);
                    to_check.insert(second);
                    nums.insert(*stone, (Some(first), Some(second)));
                } else {
                    nums.insert(*stone, (Some(*stone * 2024), None));
                    to_check.insert(*stone * 2024);
                }
            }
        }

        stones = to_check;
    }
    let mut cache: HashMap<u128, usize> = HashMap::new();

    let stones = read_file_to_vec(file_name).unwrap();
    let mut res = 0;
    for stone in stones {
        // println!("stone {}", stone);
        res += count_sub_stones(&mut cache, &nums, stone, 0, 25);
    }
    return res;
}


fn count_sub_stones(
    cache: &mut HashMap<u128, usize>,
    nums: &HashMap<usize, (Option<usize>, Option<usize>)>,
    num: usize,
    depth: usize,
    max_depth: usize
) -> usize {

    let key:u128 = (((depth as u128 )<< 64)  + num as u128).try_into().unwrap();
    if cache.contains_key(&key) {
        return cache[&key];  // Direct access with []
    }
    if depth == max_depth {
        return 1;
    }
    let (first, second) = nums[&num];
    let res;
    if second.is_none() {
        res =  count_sub_stones(cache, nums, first.unwrap(), depth + 1, max_depth);
    } else {
        res = count_sub_stones(cache, nums, first.unwrap(), depth + 1, max_depth)
            + count_sub_stones(cache, nums, second.unwrap(), depth + 1, max_depth);
    }
    cache.insert(key, res);
    return res;

}


fn part2(file_name: &str) -> usize {
    let mut stones: HashSet<usize> =
        HashSet::from_iter(read_file_to_vec(file_name).unwrap().iter().cloned());
    let mut nums: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

    for _ in 0..74 {
        let mut to_check: HashSet<usize> = HashSet::new();

        for stone in &stones {
            if *stone == 0 {
                nums.insert(*stone, (Some(1), None));
                to_check.insert(1);
            } else {
                let digits = count_digits(*stone);
                if digits % 2 == 0 {
                    let (first, second) = split_digits(*stone, digits);
                    to_check.insert(first);
                    to_check.insert(second);
                    nums.insert(*stone, (Some(first), Some(second)));
                } else {
                    nums.insert(*stone, (Some(*stone * 2024), None));
                    to_check.insert(*stone * 2024);
                }
            }
        }

        stones = to_check;
    }
    let mut cache: HashMap<u128, usize> = HashMap::new();

    let stones = read_file_to_vec(file_name).unwrap();
    let mut res = 0;
    for stone in stones {
        // println!("stone {}", stone);
        res += count_sub_stones(&mut cache, &nums, stone, 0, 75);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day11/example.txt"), 55312);
    }
    #[test]
    fn test_digit_split() {
        assert_eq!(split_digits(1234, 4), (12, 34));
    }

    #[test]
    fn test_digit_split2() {
        assert_eq!(split_digits(12, 2), (1, 2));
    }

    #[test]
    fn test_digit_split3() {
        assert_eq!(split_digits(123456, 6), (123, 456));
    }
    #[test]
    fn test_digit_count() {
        assert_eq!(count_digits(1234), 4);
    }


}
