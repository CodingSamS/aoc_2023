use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn puzzle01<P>(filename: P) -> Result<u32, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"Card\s*\d+: (.*) \| (.*)").unwrap();

    Ok(buf
        .lines()
        .filter_map(|line_result| match line_result {
            Ok(line) => match line_regex.captures(&line) {
                Some(capture) => {
                    let winning_numbers: Vec<u32> = capture[1]
                        .split(" ")
                        .map(|s| s.trim())
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect();
                    let count = capture[2]
                        .split(" ")
                        .map(|s| s.trim())
                        .filter_map(|s| s.parse::<u32>().ok())
                        .filter(|s| winning_numbers.contains(s))
                        .count() as u32;
                    if 0 < count {
                        Some(2u32.pow(count - 1))
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .sum())
}

fn puzzle02<P>(filename: P) -> Result<u32, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"Card\s*\d+: (.*) \| (.*)").unwrap();

    let mut lines_count_vec: Vec<(usize, u32)> = buf
        .lines()
        .filter_map(|line_result| match line_result {
            Ok(line) => match line_regex.captures(&line) {
                Some(capture) => {
                    let winning_numbers: Vec<u32> = capture[1]
                        .split(" ")
                        .map(|s| s.trim())
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect();
                    Some(
                        capture[2]
                            .split(" ")
                            .map(|s| s.trim())
                            .filter_map(|s| s.parse::<u32>().ok())
                            .filter(|s| winning_numbers.contains(s))
                            .count() as u32,
                    )
                }
                None => None,
            },
            Err(_) => None,
        })
        .enumerate()
        .collect();

    let mut hm: HashMap<usize, u32> = HashMap::new();
    for (i, v) in &lines_count_vec {
        hm.insert(*i, *v);
    }

    let mut sum = 0;
    while !lines_count_vec.is_empty() {
        let (mut i, mut v) = lines_count_vec.pop().unwrap();
        sum += 1;
        while 0 < v {
            v -= 1;
            i += 1;
            match hm.get(&i) {
                Some(u) => {
                    lines_count_vec.push((i, *u));
                }
                None => break,
            }
        }
    }

    Ok(sum)
}

fn main() {
    println!("Solution 1: {}", puzzle01("puzzle01_input").unwrap());
    println!("Solution 2: {}", puzzle02("puzzle01_input").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("puzzle01_input_test").unwrap(), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("puzzle01_input_test").unwrap(), 30);
    }
}
