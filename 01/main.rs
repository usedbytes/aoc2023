use std::collections::HashMap;
use std::convert::TryInto;
use std::default::Default;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn find_first_and_last<T: std::fmt::Display + Copy + Default>(
    line: String,
    values: &HashMap<&str, T>,
) -> (T, T) {
    let mut lidx: usize = line.len();
    let mut lval: T = Default::default();

    let mut ridx: usize = 0;
    let mut rval: T = Default::default();

    for (key, value) in values.into_iter() {
        match line.find(key) {
            None => continue,
            Some(idx) => {
                if idx < lidx {
                    lidx = idx;
                    lval = *value;
                }
            }
        }
        match line.rfind(key) {
            None => continue,
            Some(idx) => {
                if (idx + key.len()) > ridx {
                    ridx = idx + key.len();
                    rval = *value;
                }
            }
        }
    }

    (lval, rval)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let vmap = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut calibration_values: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = String::from(line?);

        let digits = find_first_and_last::<i8>(l, &vmap);

        let value: i32 = ((digits.0 * 10) + digits.1).try_into().unwrap();
        calibration_values.push(value);
    }

    println!("{:}", calibration_values.iter().sum::<i32>());

    Ok(())
}
