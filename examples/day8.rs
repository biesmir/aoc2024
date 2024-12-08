use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_to_char_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        matrix.push(char_vec);
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
                1 => println!("{}", part1("Inputs/Day8/input.txt")),
                2 => println!("{}", part2("Inputs/Day8/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day8/input.txt"));
        println!("{}", part2("Inputs/Day8/input.txt"));
    }
}
fn gcd(mut a: isize, mut b: isize) -> isize {
    a = a.abs();
    b = b.abs();

    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    if a == 0 {
        panic!()
    }
    a
}
fn is_antinode(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut pos: (isize, isize) = (start_pos.0 as isize, start_pos.1 as isize);
            pos.0 = i as isize;
            pos.1 = j as isize;
            if i == start_pos.0 && j == start_pos.1 {
                continue;
            }

            if pos.0 >= 0
                && pos.0 < map.len() as isize
                && pos.1 >= 0
                && pos.1 < map[0].len() as isize
            {
                let c = map[pos.0 as usize][pos.1 as usize];
                if c != '.' {
                    let search_pos = (
                        2 * pos.0 as isize - start_pos.0 as isize,
                        2 * pos.1 as isize - start_pos.1 as isize,
                    );
                    if search_pos.0 >= 0
                        && search_pos.0 < map.len() as isize
                        && search_pos.1 >= 0
                        && search_pos.1 < map[0].len() as isize
                    {
                        if map[search_pos.0 as usize][search_pos.1 as usize] == c {
                            // println!("antinote at {}:{}", start_pos.0, start_pos.1);
                            // println!(
                            //     "antena at {}:{} and {}:{}",
                            //     pos.0, pos.1, search_pos.0, search_pos.1
                            // );
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn is_antinode2(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut pos: (isize, isize) = (start_pos.0 as isize, start_pos.1 as isize);
            pos.0 = i as isize;
            pos.1 = j as isize;
            if i == start_pos.0 && j == start_pos.1 {
                continue;
            }

            if pos.0 >= 0
                && pos.0 < map.len() as isize
                && pos.1 >= 0
                && pos.1 < map[0].len() as isize
            {
                let c = map[pos.0 as usize][pos.1 as usize];
                if c != '.' {
                    let divider = gcd(
                        pos.0 as isize - start_pos.0 as isize,
                        pos.1 as isize - start_pos.1 as isize,
                    );
                    let dir = (
                        (pos.0 as isize - start_pos.0 as isize) / divider,
                        (pos.1 as isize - start_pos.1 as isize) / divider,
                    );
                    for k in 1..50 {
                        let search_pos = (
                            start_pos.0 as isize + (k * dir.0),
                            start_pos.1 as isize + (k * dir.1),
                        );
                        if search_pos == pos {
                            continue;
                        }
                        if search_pos.0 >= 0
                            && search_pos.0 < map.len() as isize
                            && search_pos.1 >= 0
                            && search_pos.1 < map[0].len() as isize
                        {
                            if map[search_pos.0 as usize][search_pos.1 as usize] == c {
                                // println!("antinote at {}:{}", start_pos.0, start_pos.1);
                                // println!(
                                //     "antena at {}:{} and {}:{}",
                                //     pos.0, pos.1, search_pos.0, search_pos.1
                                // );
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn part1(file_name: &str) -> usize {
    let map = read_file_to_char_matrix(file_name).unwrap();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_antinode(&map, (i, j)) {
                res += 1;
            }
        }
    }
    return res;
}

fn part2(file_name: &str) -> usize {
    let map = read_file_to_char_matrix(file_name).unwrap();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '.' {
                res += 1;
                continue;
            }

            if is_antinode2(&map, (i, j)) {
                res += 1;
            }
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day8/example.txt"), 14);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day8/example.txt"), 34);
    }
}
