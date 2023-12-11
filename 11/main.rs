use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();

    // Load the whole universe into memory
    for line in reader.lines() {
        let line = line?;
        let bytes = Vec::from_iter(line.bytes());
        map.push(bytes);
    }

    // Iterate once to find empty rows
    let mut empty_rows = Vec::new();
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|v| *v == b'.') {
            empty_rows.push(i);
        }
    }

    // And again to find empty columns
    let mut empty_cols = Vec::new();
    for x in 0..map[0].len() {
        let mut empty = true;
        for row in &map {
            if row[x] == b'#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(x);
        }
    }

    // And finally to find the galaxy coordinates
    let mut galaxies: Vec<(u32, u32)> = Vec::new();
    let mut y_stretch = 0;
    for (y, row) in map.iter().enumerate() {
        if y_stretch < empty_rows.len() && y > empty_rows[y_stretch] {
            y_stretch += 1;
        }

        let mut x_stretch = 0;
        for (x, cell) in row.iter().enumerate() {
            if x_stretch < empty_cols.len() && x > empty_cols[x_stretch] {
                x_stretch += 1;
            }

            if *cell == b'#' {
                galaxies.push(
                    ((x + x_stretch) as u32, (y + y_stretch) as u32)
                );
            }
        }
    }

    // Now we're looking for Manhattan distances of all pairs
    let mut distances = Vec::new();
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[i+1..].iter() {
            let distance = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
            distances.push(distance);
        }
    }

    let sum: i32 = distances.iter().sum();
    println!("{}", sum);

    Ok(())
}
