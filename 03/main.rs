use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn number_touches_symbol(
    num_coord: &(i32, i32),
    num: &String,
    symbol_coords: &HashMap::<(i32, i32), char>
) -> bool {
    let min_col = num_coord.0 - 1;
    let max_col = num_coord.0 + num.len() as i32;
    let min_row = num_coord.1 - 1;
    let max_row = num_coord.1 + 1;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            // This depends on none of the number's coordinates being
            // in symbol_coords, but why would they be?
            if symbol_coords.contains_key(&(col, row)) {
                return true;
            }
        }
    }

    return false;
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
    for (coords, number) in number_coords.into_iter() {
        let touches = number_touches_symbol(&coords, &number, &symbol_coords);
        if touches {
            sum += number.parse::<u32>().unwrap();
        }
    }

    println!("{sum}");

    Ok(())
}
