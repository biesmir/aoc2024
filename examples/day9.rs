use std::collections::HashSet;
use std::env;
use std::f32::RADIX;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut filesystem: Vec<u8> = Vec::new();
    const RADIX: u32 = 10;
    for line in reader.lines() {
        let line = line?;
        for c in line.chars() {
            filesystem.push(c.to_digit(RADIX).unwrap() as u8);
        }
    }
    Ok(filesystem)
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day9/input.txt")),
                2 => println!("{}", part2("Inputs/Day9/input.txt")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day9/input.txt"));
        println!("{}", part2("Inputs/Day9/input.txt"));
    }
}

fn decompress(filesystem: &Vec<u8>) -> Vec<Option<usize>> {
    let mut decompressed: Vec<Option<usize>> = Vec::new();
    let mut is_file = true;
    let mut file_id: usize = 0;
    for elem in filesystem.iter() {
        if is_file {
            for _ in 0..*elem {
                decompressed.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..*elem {
                decompressed.push(None);
            }
        }
        is_file = !is_file;
    }
    decompressed
}

// fn defragment(filesystem: &mut Vec<Option<usize>>) {
//     let mut skip_gap = 1;
//     let mut file_end = filesystem.len();
//     'outer: for i in 0..filesystem.len() {
//         if skip_gap > 0 {
//             skip_gap -= 1;
//             continue;
//         }

//         if filesystem[i].is_none() {
//             let mut gap_size = 0;
//             for j in i..file_end {
//                 if !filesystem[j].is_none() {
//                     break;
//                 }
//                 gap_size += 1;
//             }
//             println!("gap size at {} {}",i, gap_size);
//             let mut skip_file = 0;
//             while file_end > i{
//                 file_end -=1;

//             // for j in (i..file_end).rev() {
//                 if filesystem[file_end].is_none(){
//                     continue;
//                 }
//                 if skip_file > 0 {
//                     skip_file -= 1;
//                     continue;
//                 }
//                 println!("file is {} at {}",filesystem[file_end].unwrap(), file_end);

//                 let mut file_size = 1;
//                 for k in (i..file_end).rev() {
//                     if filesystem[k].is_none() {
//                         break;
//                     }
//                     if filesystem[k].unwrap() != filesystem[file_end].unwrap(){
//                         break;

//                     }
//                     file_size += 1;
//                 }

//                 if gap_size >= file_size {
//                     println!(" {} will fit in {}", file_size,gap_size);
//                     for k in 0..file_size {
//                         filesystem.swap(i + k, file_end - k);
//                         println!("swaping {} with {}",i + k, file_end - k);
//                     }
//                     continue 'outer;
//                 } else{
//                     println!("wont fit in {} with size {}", gap_size, file_size);
//                     skip_file = file_size-1;
//                 }

//             }
//             skip_gap = gap_size;
//         }
//  println!("filesystem {:?}", filesystem);

//     }
// }

fn defragment(filesystem: &mut Vec<Option<usize>>) {
    let mut skip_gap = 1;
    let mut last_file = 999999999;
    'outer: for i in (0..filesystem.len()).rev() {
        if !filesystem[i].is_none() {
            if filesystem[i].unwrap() == last_file{
                continue;
            }
            last_file = filesystem[i].unwrap();
            //we are in file
            let mut file_size = 1;
            for j in (0..i).rev() {
                if filesystem[j].is_none() {
                    break;
                }
                if filesystem[j].unwrap() != filesystem[i].unwrap() {
                    break;
                }
                file_size += 1;
            }

            for j in 0..i {
                if filesystem[j].is_none() {
                    //we are in gap
                    let mut gap_size = 0;
                    for k in j..filesystem.len() {
                        if !filesystem[k].is_none() {
                            break;
                        }
                        gap_size += 1;
                    }
                    if gap_size >= file_size {
                        for k in 0..file_size {
                            filesystem.swap(j+k, i-k);
                            //println!("swaping {} with {}", i + k, file_end - k);
                        }

                        //
                    }
                }
            }
        }
    }
}

fn fragment(filesystem: &mut Vec<Option<usize>>) {
    'outer: for i in 0..filesystem.len() {
        if filesystem[i].is_none() {
            for j in (i..filesystem.len()).rev() {
                if !filesystem[j].is_none() {
                    filesystem.swap(i, j);
                    continue 'outer;
                }
            }
            return;
        }
    }
}

fn checksum(filesystem: &Vec<Option<usize>>) -> usize {
    let mut res = 0;
    for i in 0..filesystem.len() {
        match filesystem[i] {
            Some(elem) => res += elem * i,
            None => continue,
        }
    }
    return res;
}

fn part1(file_name: &str) -> usize {
    let filesystem = read_file_to_vec(file_name).unwrap();
    // println!("filesystem {:?}", filesystem);
    let mut filesystem = decompress(&filesystem);
    // println!("filesystem {:?}", filesystem);
    fragment(&mut filesystem);
    // println!("filesystem {:?}", filesystem);
    return checksum(&filesystem);
}

fn part2(file_name: &str) -> usize {
    let filesystem = read_file_to_vec(file_name).unwrap();
    // println!("filesystem {:?}", filesystem);
    let mut filesystem = decompress(&filesystem);
    // println!("filesystem {:?}", filesystem);
    // println!("len {:?}", filesystem.len());
    defragment(&mut filesystem);
    // println!("filesystem {:?}", filesystem);
    return checksum(&filesystem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Inputs/Day9/example.txt"), 1928);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("Inputs/Day9/example.txt"), 2858);
    }
}
