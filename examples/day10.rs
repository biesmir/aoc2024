use debug_print::debug_println;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_matrix(filename: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let u8_vec: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        matrix.push(u8_vec);
    }

    Ok(matrix)
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day10/input.txt")),
                2 => println!("{}", part2("Inputs/Day10/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day10/input.txt"));
        println!("{}", part2("Inputs/Day10/input.txt"));
    }
}

fn reachable_9(map: &Vec<Vec<u8>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let height = map[y][x];
    if height == 9 {
        return vec![(y, x)];
    }
    let mut res: Vec<(usize, usize)> = Vec::new();

    if y + 1 < map.len() {
        if map[y + 1][x] == height + 1 {
            res.append(&mut reachable_9(map, y + 1, x));
        }
    }

    if x + 1 < map.len() {
        if map[y][x + 1] == height + 1 {
            res.append(&mut reachable_9(map, y, x + 1));
        }
    }

    if x > 0 {
        if map[y][x - 1] == height + 1 {
            res.append(&mut reachable_9(map, y, x - 1));
        }
    }

    if y > 0 {
        if map[y - 1][x] == height + 1 {
            res.append(&mut reachable_9(map, y - 1, x));
        }
    }

    return res;
}
fn score(map: &Vec<Vec<u8>>, y: usize, x: usize) -> usize {
    let nines: HashSet<(usize, usize)> = reachable_9(map, y, x).into_iter().collect();
    return nines.len();
}

fn raiting(map: &Vec<Vec<u8>>, y: usize, x: usize) -> usize {
    let height = map[y][x];
    if height == 9 {
        return 1;
    }
    let mut res = 0;

    if y + 1 < map.len() {
        if map[y + 1][x] == height + 1 {
            res += raiting(map, y + 1, x);
        }
    }

    if x + 1 < map.len() {
        if map[y][x + 1] == height + 1 {
            res += raiting(map, y, x + 1);
        }
    }

    if x > 0 {
        if map[y][x - 1] == height + 1 {
            res += raiting(map, y, x - 1);
        }
    }

    if y > 0 {
        if map[y - 1][x] == height + 1 {
            res += raiting(map, y - 1, x);
        }
    }


    return res;
}

fn part1(file_name: &str) -> usize {
    let map = read_matrix(file_name).unwrap();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                res += score(&map, i, j);
            }
        }
    }
    return res;
}

fn part2(file_name: &str) -> usize {
    let map = read_matrix(file_name).unwrap();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                res += raiting(&map, i, j);
            }
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("Inputs/Day10/example1.txt"), 1);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1("Inputs/Day10/example2.txt"), 36);
    }

    // #[test]
    // fn test_part2_1() {
    //     assert_eq!(part2("Inputs/Day10/example1.txt"), 2);
    // }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("Inputs/Day10/example2.txt"), 81);
    }
}
