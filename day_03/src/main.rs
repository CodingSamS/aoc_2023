use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const NUMBER_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

struct Number {
    value: u32,
    coordinates: Vec<(usize, usize)>,
}

struct Symbol {
    symbol: char,
    coordinate: (usize, usize),
}

fn read<P>(filename: P) -> Result<(Vec<Symbol>, Vec<Number>), &'static str>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err("Failed opening file"),
    };
    let buf = io::BufReader::new(file);

    let mut symbol_vec = Vec::new();
    let mut number_vec = Vec::new();

    for (line_id, line_result) in buf.lines().enumerate() {
        if let Ok(line) = line_result {
            let mut value_str = String::new();
            let mut coordinates = Vec::new();
            for (column_id, c) in line.chars().enumerate() {
                if NUMBER_CHARS.contains(&c) {
                    value_str.push(c);
                    coordinates.push((column_id, line_id));
                } else {
                    if let Ok(value) = value_str.parse() {
                        number_vec.push(Number { value, coordinates });
                    }
                    value_str = String::new();
                    coordinates = Vec::new();

                    if c != '.' {
                        symbol_vec.push(Symbol {
                            symbol: c,
                            coordinate: (column_id, line_id),
                        });
                    }
                }
            }
            if let Ok(value) = value_str.parse() {
                number_vec.push(Number { value, coordinates });
            }
        }
    }

    Ok((symbol_vec, number_vec))
}

fn get_neighbour(coordinate: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for x in coordinate.0.saturating_sub(1)..=coordinate.0 + 1 {
        for y in coordinate.1.saturating_sub(1)..=coordinate.1 + 1 {
            neighbours.push((x, y));
        }
    }
    neighbours
}

fn get_neighbours(coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for (x, y) in coordinates {
        neighbours.extend(get_neighbour(&(*x, *y)));
    }
    neighbours.sort();
    neighbours.dedup();
    neighbours
}

fn puzzle01(filename: &str) -> u32 {
    let (symbol_vec, number_vec) = read(filename).unwrap();
    let s_vec: Vec<(usize, usize)> = symbol_vec.iter().map(|symbol| symbol.coordinate).collect();

    number_vec
        .iter()
        .filter(|number| {
            get_neighbours(&number.coordinates)
                .iter()
                .any(|(x, y)| s_vec.contains(&(*x, *y)))
        })
        .map(|number| number.value)
        .sum()
}

fn puzzle02(filename: &str) -> u32 {
    let (symbol_vec, number_vec) = read(filename).unwrap();
    let s_vec: Vec<(usize, usize)> = symbol_vec
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| symbol.coordinate)
        .collect();

    s_vec
        .iter()
        .filter_map(|coordinate| {
            let vec: Vec<&Number> = number_vec
                .iter()
                .filter(|number| {
                    get_neighbours(&number.coordinates)
                        .iter()
                        .any(|(x, y)| coordinate == &(*x, *y))
                })
                .collect();
            if vec.len() == 2 {
                Some(vec[0].value * vec[1].value)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("Solution 1: {}", puzzle01("puzzle01_input"));
    println!("Solution 2: {}", puzzle02("puzzle01_input"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("puzzle01_input_test"), 4361)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("puzzle01_input_test"), 467835)
    }
}
