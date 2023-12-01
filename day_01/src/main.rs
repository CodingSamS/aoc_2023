use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn puzzle_01<P>(filename: P) -> Result<u32, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let l: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            format!(
                "{}{}",
                l.first().unwrap().to_string(),
                l.last().unwrap().to_string()
            )
        })
        .filter_map(|s| s.parse::<u32>().ok())
        .sum())
}

const NUMBER_STRINGS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];
const NUMBERS: [&str; 18] = [
    "1", "1", "2", "2", "3", "3", "4", "4", "5", "5", "6", "6", "7", "7", "8", "8", "9", "9",
];

fn first_and_last_digit(line: String) -> Result<String, &'static str> {
    let mut first_name_option: Option<String> = None;
    let mut last_name_option: Option<String> = None;
    let mut first_id_option: Option<usize> = None;
    let mut last_id_option: Option<usize> = None;
    for (s, n) in NUMBER_STRINGS.iter().zip(NUMBERS) {
        let f_id_option = line.find(s);
        if let Some(f_id) = f_id_option {
            match first_id_option {
                Some(first_id) => {
                    if f_id < first_id {
                        first_id_option = Some(f_id);
                        first_name_option = Some(String::from(n));
                    }
                }
                None => {
                    first_id_option = Some(f_id);
                    first_name_option = Some(String::from(n));
                }
            }
        }
        let l_id_option = line.rfind(s);
        if let Some(l_id) = l_id_option {
            match last_id_option {
                Some(last_id) => {
                    if last_id < l_id {
                        last_id_option = Some(l_id);
                        last_name_option = Some(String::from(n));
                    }
                }
                None => {
                    last_id_option = Some(l_id);
                    last_name_option = Some(String::from(n));
                }
            }
        }
    }
    if let Some(first_name) = first_name_option {
        if let Some(last_name) = last_name_option {
            return Ok(format!("{}{}", first_name, last_name));
        }
    }
    Err("No digit found")
}

fn puzzle_02<P>(filename: P) -> Result<u32, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| first_and_last_digit(line).unwrap())
        .filter_map(|s| s.parse::<u32>().ok())
        .sum())
}

fn main() {
    println!("{}", puzzle_01("puzzle01_input").unwrap());
    println!("{}", puzzle_02("puzzle01_input").unwrap());
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_01, puzzle_02};

    #[test]
    fn test1() {
        assert_eq!(puzzle_01("puzzle01_input_test").unwrap(), 142)
    }

    #[test]
    fn test2() {
        assert_eq!(puzzle_02("puzzle02_input_test").unwrap(), 281)
    }
}
