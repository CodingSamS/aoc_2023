use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

fn get_max_count(regex: &Regex, line: &String) -> Option<u32> {
    regex
        .captures_iter(&line)
        .filter_map(|caps| caps.get(1))
        .filter_map(|cap| cap.as_str().parse::<u32>().ok())
        .max()
}

fn get_games<P>(filename: P) -> Result<Vec<Game>, &'static str>
where
    P: AsRef<Path>,
{
    let game_regex = Regex::new(r"Game (\d+)").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err("Failed opening file!"),
    };
    let buf = io::BufReader::new(file);
    buf.lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let blue_option = get_max_count(&blue_regex, &line);
            let red_option = get_max_count(&red_regex, &line);
            let green_option = get_max_count(&green_regex, &line);
            match game_regex.captures(&line) {
                Some(caps) => match caps[1].parse::<u32>() {
                    Ok(id) => match (blue_option, red_option, green_option) {
                        (Some(blue), Some(red), Some(green)) => Ok(Game {
                            id,
                            blue,
                            red,
                            green,
                        }),
                        _ => Err("Some cube colours are missing"),
                    },
                    Err(_) => Err("Game ID is no valid u32"),
                },
                None => Err("No Game ID found"),
            }
        })
        .collect()
}

fn puzzle01(
    filename: &str,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
) -> Result<u32, &'static str> {
    Ok(get_games(filename)?
        .into_iter()
        .filter_map(|game| {
            if game.blue <= max_blue && game.red <= max_red && game.green <= max_green {
                Some(game.id)
            } else {
                None
            }
        })
        .sum())
}

fn puzzle02(filename: &str) -> Result<u32, &'static str> {
    Ok(get_games(filename)?
        .into_iter()
        .map(|game| game.blue * game.red * game.green)
        .sum())
}

fn main() {
    println!(
        "Solution 1: {}",
        puzzle01("puzzle01_input", 14, 12, 13).unwrap()
    );
    println!("Solution 2: {}", puzzle02("puzzle01_input").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(
            crate::puzzle01("puzzle01_input_test", 14, 12, 13).unwrap(),
            8
        )
    }
    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("puzzle01_input_test").unwrap(), 2286)
    }
}
