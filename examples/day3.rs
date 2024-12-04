use std::fs::File;
use std::env;
use std::io::{self, Read};

fn read_file_to_string(filepath: &str) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day3/input.txt")),
                2 => println!("{}", part2("Inputs/Day3/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day3/input.txt"));
        println!("{}", part2("Inputs/Day3/input.txt"));
    }
}


fn try_mul(line: &str, i: usize) -> usize {
    let mul = line.get(i..).unwrap();
    if let Some(end) = mul.find(")") {
        let mul = mul.get(4..end).unwrap();
        return mul
            .split_once(',')
            .and_then(|(a, b)| Some(a.parse::<usize>().ok()? * b.parse::<usize>().ok()?))
            .unwrap_or(0);
    }
    0
}

fn part1(file_name: &str) -> usize {
    let content = read_file_to_string(file_name).unwrap();
    let muls: Vec<usize> = content
        .match_indices("mul(")
        .map(|(index, _)| index)
        .collect();
    let mut res = 0;
    for i in muls {
        res += try_mul(&content, i);
    }
    return res;
}
fn part2(file_name: &str) -> usize {
    let content = read_file_to_string(file_name).unwrap();
    let muls: Vec<usize> = content
        .match_indices("mul(")
        .map(|(index, _)| index)
        .collect();
    let dos: Vec<usize> = content
        .match_indices("do()")
        .map(|(index, _)| index)
        .collect();
    let donts: Vec<usize> = content
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect();

    let mut disable_zones: Vec<(usize, usize)> = Vec::new();
    let mut iter = dos.iter();
    for dont in donts {
        if !disable_zones.is_empty() {
            if dont < disable_zones[disable_zones.len() - 1].1 {
                continue;
            }
        }
        let start: usize = dont;
        let end: usize;
        loop {
            match iter.next() {
                Some(value) => {
                    if *value > start {
                        end = *value;
                        break;
                    }
                }
                None => {
                    end = 99999999;
                    break;
                }
            }
        }
        disable_zones.push((start, end));
    }
    let mut i = muls.len();
    let mut new_muls: Vec<usize> = Vec::new();

    while i > 0 {
        i -= 1;
        let mut keep = true;
        for ex in &disable_zones {
            //println!("dont {} do {}", ex.0, ex.1);
            if ex.0 < muls[i] && ex.1 > muls[i] {
                keep = false;
            }
        }
        if keep {
            new_muls.push(muls[i]);
        }
    }

    let mut res = 0;
    for i in new_muls {
        res += try_mul(&content, i);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day3/example.txt"), 161);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day3/example2.txt"), 48);
    }
}
