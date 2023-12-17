use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut platform = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row = Vec::from_iter(line.chars());
        platform.push(row);
    }

    let nrows = platform.len();
    let ncols = platform[0].len();

    let mut last_blockage = vec![0; ncols];
    let mut load = 0;
    for (i, row) in platform.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            match val {
                '#' => {
                    last_blockage[j] = i + 1;
                },
                'O' => {
                    load += nrows - last_blockage[j];
                    last_blockage[j] += 1;
                },
                '.' => {
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }

    println!("{load}");

    Ok(())
}
