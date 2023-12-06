use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

const NUMBER_REGEX: Regex = Regex::new(r"(\d+)").unwrap();

fn parse_mapping_section<F>(
    section_name: &str,
    buf: &mut Lines<BufReader<File>>,
) -> Result<Vec<F>, String>
where
    F: Fn(u32) -> u32,
{
    match buf.next() {
        Some(result) => match result {
            Ok(line) => {
                if line == "seed-to-soil map:" {
                    let mut seed_to_soil_vec_temp = Vec::new();
                    loop {
                        match buf.next() {
                            Some(inner_result) => match inner_result {
                                Ok(inner_line) => NUMBER_LINE_REGEX.captures_iter(&inner_line),
                                Err(_) => {
                                    Err(format!("{}: Error parsing number line", section_name))
                                }
                            },
                            None => Err(format!("{}: Number line does not exist", section_name)),
                        }
                        break;
                    }
                    Ok(seed_to_soil_vec_temp)
                } else {
                    Err(format!("{}: line does not match", section_name))
                }
            }
            Err(_) => Err(format!("{}: Could not parse line!", section_name)),
        },
        None => Err(format!("{}: Line does not exist!", section_name)),
    }
}

fn puzzle01<P>(filename: P) -> Result<u32, String>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Error opening file!")),
    };
    let mut buf = BufReader::new(file).lines();

    // read in seeds:
    let seed_regex = Regex::new(r"seeds:( \d)+").unwrap();
    let seed_vec: Vec<u32> = match buf.next() {
        Some(result) => match result {
            Ok(line) => match seed_regex.captures(&line) {
                Some(capture) => {
                    let mut capture_iter = capture.iter();
                    capture_iter.next();
                    capture_iter
                        .filter_map(|option_c| match option_c {
                            Some(c) => Some(c.as_str().trim()),
                            None => None,
                        })
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect()
                }
                None => return Err(String::from("Seed line does not match the pattern!")),
            },
            Err(_) => return Err(String::from("Could not parse seed line!")),
        },
        None => return Err(String::from("Seed line does not exist!")),
    };

    buf.next();

    let seed_to_soil_vec = {}?;
}

fn main() {
    println!("Hello, world!");
}
