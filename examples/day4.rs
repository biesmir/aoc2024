use std::fs::File;

use std::io::{self, BufRead, BufReader, Read};

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
    println!("got: {}", part1("Inputs/Day4/input.txt"));
    println!("got: {}", part2("Inputs/Day4/input.txt"));
}

fn check_xmas(matrix: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let mut res = 0;
    if pos.0 >= 3 {
        if matrix[pos.0 - 1][pos.1] == 'M'
            && matrix[pos.0 - 2][pos.1] == 'A'
            && matrix[pos.0 - 3][pos.1] == 'S'
        {
            res += 1;
        }
    }
    if pos.0 >= 3 && pos.1 >= 3 {
        if matrix[pos.0 - 1][pos.1 - 1] == 'M'
            && matrix[pos.0 - 2][pos.1 - 2] == 'A'
            && matrix[pos.0 - 3][pos.1 - 3] == 'S'
        {
            res += 1;
        }
    }
    if pos.1 >= 3 {
        if matrix[pos.0][pos.1 - 1] == 'M'
            && matrix[pos.0][pos.1 - 2] == 'A'
            && matrix[pos.0][pos.1 - 3] == 'S'
        {
            res += 1;
        }
    }
    if pos.0 <= matrix.len() - 4 {
        if matrix[pos.0 + 1][pos.1] == 'M'
            && matrix[pos.0 + 2][pos.1] == 'A'
            && matrix[pos.0 + 3][pos.1] == 'S'
        {
            res += 1;
        }
    }

    if pos.0 <= matrix.len() - 4 && pos.1 <= matrix[0].len() - 4 {
        if matrix[pos.0 + 1][pos.1 + 1] == 'M'
            && matrix[pos.0 + 2][pos.1 + 2] == 'A'
            && matrix[pos.0 + 3][pos.1 + 3] == 'S'
        {
            res += 1;
        }
    }

    if pos.1 <= matrix[0].len() - 4 {
        if matrix[pos.0][pos.1 + 1] == 'M'
            && matrix[pos.0][pos.1 + 2] == 'A'
            && matrix[pos.0][pos.1 + 3] == 'S'
        {
            res += 1;
        }
    }

    if pos.0 >= 3 && pos.1 <= matrix[0].len() - 4 {
        if matrix[pos.0 - 1][pos.1 + 1] == 'M'
            && matrix[pos.0 - 2][pos.1 + 2] == 'A'
            && matrix[pos.0 - 3][pos.1 + 3] == 'S'
        {
            res += 1;
        }
    }

    if pos.0 <= matrix.len() - 4 && pos.1 >= 3 {
        if matrix[pos.0 + 1][pos.1 - 1] == 'M'
            && matrix[pos.0 + 2][pos.1 - 2] == 'A'
            && matrix[pos.0 + 3][pos.1 - 3] == 'S'
        {
            res += 1;
        }
    }

    res
}

fn check_x_mas(matrix: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let mut res = 0;
    if pos.0 >= 1 && pos.1 >= 1 && pos.0 <= matrix.len() - 2 && pos.1 <= matrix[0].len() - 2 {
        if matrix[pos.0 - 1][pos.1 - 1] == 'M' && matrix[pos.0 + 1][pos.1 + 1] == 'S' {
            res += 1;
        }
        if matrix[pos.0 + 1][pos.1 - 1] == 'M' && matrix[pos.0 - 1][pos.1 + 1] == 'S' {
            res += 1;
        }
        if matrix[pos.0 + 1][pos.1 + 1] == 'M' && matrix[pos.0 - 1][pos.1 - 1] == 'S' {
            res += 1;
        }
        if matrix[pos.0 - 1][pos.1 + 1] == 'M' && matrix[pos.0 + 1][pos.1 - 1] == 'S' {
            res += 1;
        }
    }

    if res == 2 {
        return 1;
    } else {
        return 0;
    }
}

fn part2(file_name: &str) -> usize {
    let content = read_file_to_char_matrix(file_name).unwrap();
    let mut a_list: Vec<(usize, usize)> = Vec::new();
    for i in 0..content.len() {
        for j in 0..content[0].len() {
            if content[i][j] == 'A' {
                a_list.push((i, j));
            }
        }
    }

    a_list.into_iter().map(|x| check_x_mas(&content, x)).sum()
}

fn part1(file_name: &str) -> usize {
    let content = read_file_to_char_matrix(file_name).unwrap();
    let mut x_list: Vec<(usize, usize)> = Vec::new();
    for i in 0..content.len() {
        for j in 0..content[0].len() {
            if content[i][j] == 'X' {
                x_list.push((i, j));
            }
        }
    }

    x_list.into_iter().map(|x| check_xmas(&content, x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day4/example.txt"), 18);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day4/example.txt"), 9);
    }
}
