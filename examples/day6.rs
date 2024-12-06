use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_to_char_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Create a vector to store our matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        // Convert each line into a vector of characters
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
                1 => println!("{}", part1("Inputs/Day6/input.txt")),
                2 => println!("{}", part2("Inputs/Day6/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day6/input.txt"));
        println!("{}", part2("Inputs/Day6/input.txt"));
    }
}

fn turn_right(pos: &mut ((usize, usize), (isize, isize))) {
    if pos.1 .0 == 0 && pos.1 .1 == 1 {
        *pos = ((pos.0 .0, pos.0 .1), (1, 0));
    } else if pos.1 .0 == 1 && pos.1 .1 == 0 {
        *pos = ((pos.0 .0, pos.0 .1), (0, -1));
    } else if pos.1 .0 == 0 && pos.1 .1 == -1 {
        *pos = ((pos.0 .0, pos.0 .1), (-1, 0));
    } else if pos.1 .0 == -1 && pos.1 .1 == 0 {
        *pos = ((pos.0 .0, pos.0 .1), (0, 1));
    } else {
        panic!();
    }
}

fn one_step(map: &Vec<Vec<char>>, pos: &mut ((usize, usize), (isize, isize))) -> bool {
    let newposy: isize = pos.0 .0 as isize + pos.1 .0;
    let newposx: isize = pos.0 .1 as isize + pos.1 .1;

    if newposy < 0 {
        return false;
    } else if newposx < 0 {
        return false;
    } else if newposy >= map.len() as isize {
        return false;
    } else if newposx >= map[0].len() as isize {
        return false;
    } else if map[newposy as usize][newposx as usize] == '#' {
        turn_right(pos);
        return true;
    } else {
        pos.0 .0 = newposy.try_into().unwrap();
        pos.0 .1 = newposx.try_into().unwrap();
        return true;
    }
}

fn part1(file_name: &str) -> usize {
    let matrix = read_file_to_char_matrix(file_name).unwrap();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // let mut positions: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut start_pos: ((usize, usize), (isize, isize)) = ((0, 0), (0, 0));
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '^' {
                start_pos = ((i, j), (-1, 0));
                visited.insert((i, j));
                // positions.insert(start_pos);
                break;
            }
        }
    }
    let start_pos = start_pos;
    let mut pos = start_pos;
    let newposy: isize = pos.0 .0 as isize + pos.1 .0;
    let newposx: isize = pos.0 .1 as isize + pos.1 .1;
    pos.0 .0 = newposy.try_into().unwrap();
    pos.0 .1 = newposx.try_into().unwrap();
    visited.insert((pos.0 .0, pos.0 .1));

    while one_step(&matrix, &mut pos) {
        // positions.insert(pos);

        visited.insert((pos.0 .0, pos.0 .1));
        // println!("pos {:?}", pos);
    }

    return visited.len();
}

fn is_looped(matrix: &Vec<Vec<char>>) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut positions: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut start_pos: ((usize, usize), (isize, isize)) = ((0, 0), (0, 0));
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '^' {
                start_pos = ((i, j), (-1, 0));
                visited.insert((i, j));
                positions.insert(start_pos);
                break;
            }
        }
    }
    let start_pos = start_pos;
    let mut pos = start_pos;
    let newposy: isize = pos.0 .0 as isize + pos.1 .0;
    let newposx: isize = pos.0 .1 as isize + pos.1 .1;
    pos.0 .0 = newposy.try_into().unwrap();
    pos.0 .1 = newposx.try_into().unwrap();
    visited.insert((pos.0 .0, pos.0 .1));

    while one_step(&matrix, &mut pos) {
        if positions.contains(&pos) {
            return true;
        }
        positions.insert(pos);
        visited.insert((pos.0 .0, pos.0 .1));
    }
    return false;
}

fn get_original_path(matrix: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut positions: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut start_pos: ((usize, usize), (isize, isize)) = ((0, 0), (0, 0));
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '^' {
                start_pos = ((i, j), (-1, 0));
                positions.insert(start_pos);
                break;
            }
        }
    }
    let start_pos = start_pos;
    let mut pos = start_pos;
    let newposy: isize = pos.0 .0 as isize + pos.1 .0;
    let newposx: isize = pos.0 .1 as isize + pos.1 .1;
    pos.0 .0 = newposy.try_into().unwrap();
    pos.0 .1 = newposx.try_into().unwrap();
    visited.insert((pos.0 .0, pos.0 .1));

    while one_step(&matrix, &mut pos) {
        positions.insert(pos);
        visited.insert((pos.0 .0, pos.0 .1));
    }
    return visited;
}

fn part2(file_name: &str) -> usize {
    let matrix = read_file_to_char_matrix(file_name).unwrap();
    let mut res = 0;

    let to_check = get_original_path(&matrix);

    for field in to_check {
        let i = field.0;
        let j = field.1;

        if matrix[i][j] == '.' {
            let mut map = matrix.clone();
            map[i][j] = '#';
            if is_looped(&map) {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day6/example.txt"), 41);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day6/example.txt"), 6);
    }
}
