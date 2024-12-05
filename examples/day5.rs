use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_to_char_matrix(filename: &str) -> io::Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut matrix: Vec<Vec<usize>> = Vec::new();
    let mut manual: Vec<(usize, usize)> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let numbers: Vec<usize> = line.split('|').filter_map(|s| s.parse().ok()).collect();

        if numbers.len() == 2 {
            manual.push((numbers[0], numbers[1]));
        }
    }

    for line in lines {
        if let Ok(line) = line {
            let row: Vec<usize> = line
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !row.is_empty() {
                matrix.push(row);
            }
        }
    }

    Ok((manual, matrix))
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day5/input.txt")),
                2 => println!("{}", part2("Inputs/Day5/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day5/input.txt"));
        println!("{}", part2("Inputs/Day5/input.txt"));
    }
}

fn check_line(manual: &Vec<(usize, usize)>, line: &Vec<usize>) -> usize {
    for rule in manual {
        // println!("rule {} {}", rule.0, rule.1);
        let mut second_found = false;
        for num in line {
            if *num == rule.1 {
                second_found = true;
            } else if *num == rule.0 {
                if second_found {
                    // println!("{} before {}", rule.1, rule.0);
                    return 0;
                }
            }
        }
    }
    // println!("return {}", line[line.len()/2]);
    return line[line.len() / 2];
}

fn part1(file_name: &str) -> usize {
    let (manual, matrix) = read_file_to_char_matrix(file_name).unwrap();
    matrix.iter().map(|line| check_line(&manual, line)).sum()
}

fn insert_into_ordering(manual: &Vec<(usize, usize)>, ordering: &mut Vec<usize>, num: usize) {
    if ordering.is_empty() {
        ordering.push(num);
        return;
    }
    let mut new_index = ordering.len();
    for rule in manual {
        if rule.0 == num {
            if let Some(index) = ordering.iter().position(|&r| r == rule.1) {
                if new_index > index {
                    new_index = index;
                    // println!("new index {}", new_index);
                }
            }
        } else if rule.1 == num {
            if let Some(index) = ordering.iter().position(|&r| r == rule.0) {
                if new_index < index {
                    new_index = index + 1;
                    // println!("newindex {}", new_index);
                }
            }
        }
    }
    ordering.insert(new_index, num);
}

fn sorted_line(manual: &Vec<(usize, usize)>, line: &Vec<usize>) -> Vec<usize> {
    let mut ordering = Vec::new();

    for num in line {
        insert_into_ordering(manual, &mut ordering, *num);
        // println!("ordering is {:?}", ordering);
        if 0 == check_line(manual, &ordering) {
            panic!();
        }
    }

    return ordering;
}

fn part2(file_name: &str) -> usize {
    let (mut manual, matrix) = read_file_to_char_matrix(file_name).unwrap();
    manual.sort_by_key(|k| k.1);
    let mut res = 0;

    // println!("ordering is {:?}", create_ordering(&manual));
    for line in matrix {
        if 0 == check_line(&manual, &line) {
            res += check_line(&manual, &sorted_line(&manual, &line));

        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day5/example.txt"), 143);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day5/example.txt"), 123);
    }
}
