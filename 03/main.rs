use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::option::Option;

// Only returns the fist symbol found
fn number_touches_symbol(
    num_coord: &(i32, i32),
    num: &String,
    symbol_coords: &HashMap::<(i32, i32), char>
) -> Option<((i32, i32), char)> {
    let min_col = num_coord.0 - 1;
    let max_col = num_coord.0 + num.len() as i32;
    let min_row = num_coord.1 - 1;
    let max_row = num_coord.1 + 1;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            // This depends on none of the number's coordinates being
            // in symbol_coords, but why would they be?
            match symbol_coords.get(&(col, row)) {
                Some(c) => return Some(((col, row), *c)),
                None => {},
            }
        }
    }

    return None;
}

fn find_gears(
    number_coords: &HashMap::<(i32, i32), String>,
    symbol_coords: &HashMap::<(i32, i32), char>
) -> Vec<(u32, u32)> {
    let mut gear_candidates = HashMap::<(i32, i32), Vec<String>>::new();

    for (coord, number) in number_coords.iter() {
        // This assumes that a gear number _only_ touches the 
        // gear. If it touches another symbol, then we might hit that
        // first and number_touches_symbol will return that instead.
        match number_touches_symbol(&coord, &number, symbol_coords) {
            Some((symbol_coord, '*')) => {
                gear_candidates.entry(symbol_coord)
                    .or_insert_with(Vec::<String>::new)
                    .push(number.to_string());
            },
            _ => (),
        }
    }

    let mut gears = Vec::new();
    for (_, numbers) in gear_candidates.iter() {
        if numbers.len() == 2 {
            gears.push((
                    numbers[0].parse::<u32>().unwrap(),
                    numbers[1].parse::<u32>().unwrap(),
            ));
        }
    }

    return gears;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut symbol_coords = HashMap::<(i32, i32), char>::new();
    let mut number_coords = HashMap::<(i32, i32), String>::new();

    for (row, l) in reader.lines().enumerate() {
        let line = l?;
        let mut number = String::new();
        for (col, letter) in line.chars().enumerate() {
            if letter.is_ascii_digit() {
                number.push(letter);
            } else {
                if letter != '.' {
                    symbol_coords.insert((col as i32, row as i32), letter);
                }

                if number.len() > 0 {
                    number_coords.insert(
                        ((col - number.len()) as i32, row as i32),
                        number,
                    );

                    number = String::new();
                }
            }
        }

        if number.len() > 0 {
            number_coords.insert(
                ((&line.len() - number.len()) as i32, row as i32),
                number,
            );
        }
    }

    let mut sum = 0;
    for (coords, number) in number_coords.iter() {
        match number_touches_symbol(&coords, &number, &symbol_coords) {
            Some(_) => {
                sum += number.parse::<u32>().unwrap();
            },
            None => {},
        }
    }

    println!("{sum}");

    let gear_sum: u32 = find_gears(&number_coords, &symbol_coords)
        .iter()
        .map(|g| { g.0 * g.1 })
        .sum();

    println!("{gear_sum}");

    Ok(())
}
