use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn find_galaxies(
    map: &Vec<Vec<u8>>,
    empty_cols: &Vec<usize>,
    empty_rows: &Vec<usize>,
    stretch_factor: usize,
) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
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
                    (
                        x + x_stretch * (stretch_factor - 1),
                        y + y_stretch * (stretch_factor - 1),
                    )
                );
            }
        }
    }

    return galaxies;
}

fn sum_manhattan_distances_pairwise(galaxies: &Vec<(usize, usize)>) -> u64 {
    let mut distances = Vec::new();
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[i+1..].iter() {
            let distance = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
            distances.push(distance as u64);
        }
    }

    return distances.iter().sum();
}

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

    // Stretch factor 2 for part 1
    let p1_galaxies = find_galaxies(&map, &empty_cols, &empty_rows, 2);
    let p1_sum = sum_manhattan_distances_pairwise(&p1_galaxies);
    println!("{}", p1_sum);

    // Stretch factor 1000000 for part 2!
    let p2_galaxies = find_galaxies(&map, &empty_cols, &empty_rows, 1000000);
    let p2_sum = sum_manhattan_distances_pairwise(&p2_galaxies);
    println!("{}", p2_sum);

    Ok(())
}
