use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn access_row_major(outer: usize, inner: usize, platform: &mut Vec<Vec<char>>)
    -> &mut char {
    return &mut platform[outer][inner];
}

fn access_col_major(outer: usize, inner: usize, platform: &mut Vec<Vec<char>>)
    -> &mut char {
    return &mut platform[inner][outer];
}

fn tilt_platform(
    platform: &mut Vec<Vec<char>>,
    outer: &Vec<usize>,
    inner: &Vec<usize>,
    access: fn(usize, usize, &mut Vec<Vec<char>>) -> &mut char,
    blockages: &mut Vec<usize>,
    block_delta: i32,
) {
    for i in outer {
        let i = *i;
        for j in inner {
            let j = *j;
            let val = access(i, j, platform);
            match val {
                '#' => {
                    blockages[j] = (i as i32 + block_delta) as usize;
                },
                'O' => {
                    // Update vacated square
                    *val = '.';

                    let last = blockages[j];

                    // Move rock
                    *access(last, j, platform) = 'O';

                    blockages[j] = ((blockages[j] as i32) + block_delta) as usize;
                },
                '.' => {
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    let nrows = platform.len();
    let ncols = platform[0].len();
    tilt_platform(platform, &(0..nrows).collect(), &(0..ncols).collect(), access_row_major,
                  &mut vec![0; ncols], 1);
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
    let nrows = platform.len();
    let ncols = platform[0].len();
    tilt_platform(platform, &(0..nrows).rev().collect(), &(0..ncols).collect(), access_row_major,
                  &mut vec![nrows - 1; ncols], -1);
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
    let nrows = platform.len();
    let ncols = platform[0].len();
    tilt_platform(platform, &(0..ncols).collect(), &(0..nrows).collect(), access_col_major,
                  &mut vec![0; nrows], 1);
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
    let nrows = platform.len();
    let ncols = platform[0].len();
    tilt_platform(platform, &(0..ncols).rev().collect(), &(0..nrows).collect(), access_col_major,
                  &mut vec![ncols - 1; nrows], -1);
}

fn do_cycle(platform: &mut Vec<Vec<char>>) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn calc_load(platform: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let nrows = platform.len();

    for (i, row) in platform.iter().enumerate() {
        for val in row.iter() {
            if *val == 'O' {
                load += nrows - i;
            }
        }
    }

    return load;
}

/*
fn print_platform(platform: &Vec<Vec<char>>) {
    for row in platform.iter() {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}
*/

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

    let mut loads = Vec::new();
    let mut cycle_length = 0;
    let mut cycle_start = 0;

    for i in 0..1000 {
        do_cycle(&mut platform);
        let load = calc_load(&platform);

        if let Some(pos) = loads.iter().rposition(|&x| x  == load) {
            let length = loads.len() - pos;
            if pos > length * 2 {
                let mut ok = true;
                for j in pos - (length * 2)..pos {
                    let v1 = loads[j];
                    let v2 = loads[j + length];

                    if v1 != v2 {
                        ok = false;
                    }
                }

                if ok {
                    cycle_length = length;
                    cycle_start = i - length;
                    break;
                }
            }
        }

        loads.push(load);
    }

    assert!(cycle_length != 0);

    let goal = 1000000000;
    let offset = (goal - cycle_start) % cycle_length;
    println!("{}", loads[cycle_start + offset - 1]);

    Ok(())
}
