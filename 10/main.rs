use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn trace_path(
    map: &HashMap<(i32, i32), &&str>,
    start: &(i32, i32)
) -> Vec<(i32, i32)> {
    let dirs_arrivals = [
        ((1, 0), 'W'),
        ((0, 1), 'N'),
        ((-1, 0), 'E'),
        ((0, -1), 'S'),
    ];

    let mut path = Vec::new();

    let mut going = 'x';
    let mut cell = (0, 0);
    for d in dirs_arrivals {
        cell = (start.0 + (d.0).0, start.1 + (d.0).1);
        if let Some(pipe) = map.get(&cell) {
            if let Some(idx) = pipe.find(d.1) {
                    going = pipe.chars().nth((idx + 1) % 2).unwrap();
                    cell = cell;
                    path.push(cell);
                    break;
            }
        }
    }

    let move_dirs = HashMap::from([
        ('N', ((0, -1), 'S')),
        ('E', ((1, 0), 'W')),
        ('S', ((0, 1), 'N')),
        ('W', ((-1, 0), 'E')),
    ]);

    loop {
        let dir = move_dirs.get(&going).unwrap();
        cell = (cell.0 + (dir.0).0, cell.1 + (dir.0).1);
        if cell == *start {
            break;
        } else if let Some(pipe) = map.get(&cell) {
            path.push(cell);

            let idx = pipe.find(dir.1).unwrap();
            going = pipe.chars().nth((idx + 1) % 2).unwrap();
        }
    }

    return path;
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let pipe_parse = HashMap::from([
        ('|', "NS"),
        ('-', "EW"),
        ('L', "NE"),
        ('J', "NW"),
        ('7', "SW"),
        ('F', "SE"),
    ]);

    let mut map = HashMap::new();
    let mut start = (0, 0);

    for (row, l) in reader.lines().enumerate() {
        let line = l?;
        for (col, letter) in line.chars().enumerate() {
            if let Some(c) = pipe_parse.get(&letter) {
                map.insert((col as i32, row as i32), c);
            } else if letter == 'S' {
                start = (col as i32, row as i32);
            }
        }
    }

    let path = trace_path(&map, &start);
    println!("{}", (path.len() + 1) / 2);

    Ok(())
}
