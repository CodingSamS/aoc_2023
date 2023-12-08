use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

lazy_static! {
    static ref NUMBER_LINE_REGEX: Regex = Regex::new(r"(\d+)\s(\d+)\s(\d+)").unwrap();
}

fn parse_mapping_section(
    section_name: &str,
    buf: &mut Lines<BufReader<File>>,
) -> Result<Vec<impl Fn(u64) -> Option<u64>>, String> {
    let Some(Ok(section_line)) = buf.next() else {
        return Err(format!("{} Could not parse section line", section_name));
    };
    if section_line == section_name {
        let mut seed_to_soil_vec = Vec::new();
        loop {
            let Some(Ok(number_line)) = buf.next() else {
                break;
            };
            match NUMBER_LINE_REGEX.captures(&number_line) {
                Some(capture) => {
                    let Ok(destination_range_start) = capture[1].parse::<u64>() else {
                        return Err(format!(
                            "{} Error parsing destination range start",
                            section_name
                        ));
                    };
                    let Ok(source_range_start) = capture[2].parse::<u64>() else {
                        return Err(format!("{} Error parsing source range start", section_name));
                    };
                    let Ok(range_length) = capture[3].parse::<u64>() else {
                        return Err(format!("{} Error parsing range length", section_name));
                    };
                    let closure = move |input: u64| {
                        if (source_range_start..source_range_start + range_length).contains(&input)
                        {
                            Some(input - source_range_start + destination_range_start)
                        } else {
                            None
                        }
                    };
                    seed_to_soil_vec.push(closure);
                }
                None => break,
            }
        }
        Ok(seed_to_soil_vec)
    } else {
        Err(format!("{} section line does not match", section_name))
    }
}

fn puzzle01<P>(filename: P) -> Result<u64, String>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Error opening file")),
    };
    let mut buf = BufReader::new(file).lines();

    // read in seeds:
    let seed_regex = Regex::new(r"(\d+)").unwrap();
    let seed_vec: Vec<u64> = match buf.next() {
        Some(result) => match result {
            Ok(line) => seed_regex
                .captures_iter(&line)
                .filter_map(|c| c[1].parse::<u64>().ok())
                .collect(),
            Err(_) => return Err(String::from("Could not parse seed line")),
        },
        None => return Err(String::from("Seed line does not exist")),
    };

    buf.next();

    let mapping_routine_vec = vec![
        (
            parse_mapping_section("seed-to-soil map:", &mut buf)?,
            "seed-to-soil map",
        ),
        (
            parse_mapping_section("soil-to-fertilizer map:", &mut buf)?,
            "soil-to-fertilizer",
        ),
        (
            parse_mapping_section("fertilizer-to-water map:", &mut buf)?,
            "fertilizer-to-water",
        ),
        (
            parse_mapping_section("water-to-light map:", &mut buf)?,
            "water-to-light",
        ),
        (
            parse_mapping_section("light-to-temperature map:", &mut buf)?,
            "light-to-temperature",
        ),
        (
            parse_mapping_section("temperature-to-humidity map:", &mut buf)?,
            "temperature-to-humidity",
        ),
        (
            parse_mapping_section("humidity-to-location map:", &mut buf)?,
            "humidity-to-location",
        ),
    ];

    let mut lowest_location_number: Option<u64> = None;
    for seed in seed_vec {
        let mut location_number = seed;
        for (map_vec, map_vec_name) in &mapping_routine_vec {
            let mut v: Vec<u64> = map_vec.iter().filter_map(|f| f(location_number)).collect();
            if v.len() == 1 {
                location_number = v.pop().unwrap();
            } else if 1 < v.len() {
                return Err(format!(
                    "There was more than one function mapping for the seed. Number of mappings: {}. Mapping Name: {}", v.len(), map_vec_name
                ));
            }
        }
        match lowest_location_number {
            Some(number) => {
                if location_number < number {
                    lowest_location_number = Some(location_number);
                }
            }
            None => lowest_location_number = Some(location_number),
        }
    }
    match lowest_location_number {
        Some(number) => Ok(number),
        None => Err(String::from("no lowest location number found")),
    }
}

fn puzzle02<P>(filename: P) -> Result<u64, String>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Error opening file")),
    };
    let mut buf = BufReader::new(file).lines();

    // read in seeds:
    let seed_regex = Regex::new(r"(\d+) (\d+)").unwrap();
    let seed_vec: Vec<(u64, u64)> = match buf.next() {
        Some(result) => match result {
            Ok(line) => seed_regex
                .captures_iter(&line)
                .filter_map(|c| {
                    let c1 = c[1].parse::<u64>();
                    let c2 = c[2].parse::<u64>();
                    if c1.is_ok() && c2.is_ok() {
                        Some((c1.unwrap(), c2.unwrap()))
                    } else {
                        None
                    }
                })
                .collect(),
            Err(_) => return Err(String::from("Could not parse seed line")),
        },
        None => return Err(String::from("Seed line does not exist")),
    };

    buf.next();

    let mapping_routine_vec = vec![
        (
            parse_mapping_section("seed-to-soil map:", &mut buf)?,
            "seed-to-soil map",
        ),
        (
            parse_mapping_section("soil-to-fertilizer map:", &mut buf)?,
            "soil-to-fertilizer",
        ),
        (
            parse_mapping_section("fertilizer-to-water map:", &mut buf)?,
            "fertilizer-to-water",
        ),
        (
            parse_mapping_section("water-to-light map:", &mut buf)?,
            "water-to-light",
        ),
        (
            parse_mapping_section("light-to-temperature map:", &mut buf)?,
            "light-to-temperature",
        ),
        (
            parse_mapping_section("temperature-to-humidity map:", &mut buf)?,
            "temperature-to-humidity",
        ),
        (
            parse_mapping_section("humidity-to-location map:", &mut buf)?,
            "humidity-to-location",
        ),
    ];

    let mut lowest_location_number: Option<u64> = None;
    for (seed_start, seed_range) in seed_vec {
        println!("Starting seed: {} Range: {}", seed_start, seed_range);
        for seed_num in seed_start..seed_start + seed_range {
            let mut location_number = seed_num;
            for (map_vec, map_vec_name) in &mapping_routine_vec {
                let mut v: Vec<u64> = map_vec.iter().filter_map(|f| f(location_number)).collect();
                if v.len() == 1 {
                    location_number = v.pop().unwrap();
                } else if 1 < v.len() {
                    return Err(format!(
                    "There was more than one function mapping for the seed. Number of mappings: {}. Mapping Name: {}", v.len(), map_vec_name
                ));
                }
            }
            match lowest_location_number {
                Some(number) => {
                    if location_number < number {
                        lowest_location_number = Some(location_number);
                    }
                }
                None => lowest_location_number = Some(location_number),
            }
        }
    }
    match lowest_location_number {
        Some(number) => Ok(number),
        None => Err(String::from("no lowest location number found")),
    }
}

fn main() {
    println!("Solution: {}", puzzle01("puzzle01_input").unwrap());
    println!("Solution: {}", puzzle02("puzzle01_input").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("puzzle01_input_test").unwrap(), 35)
    }
    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("puzzle01_input_test").unwrap(), 46)
    }
}
