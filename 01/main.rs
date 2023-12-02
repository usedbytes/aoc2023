use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut calibration_values: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let mut digits: (i8, i8) = (-1, -1);

        for letter in l.chars() {
            if letter.is_digit(10) {
                let n: i8 = letter.to_digit(10).unwrap().try_into().unwrap();
                if digits.0 < 0 {
                    digits.0 = n;
                }
                digits.1 = n;
            }
        }

        let value: i32 = ((digits.0 * 10) + digits.1).try_into().unwrap();
        calibration_values.push(value);
    }

    println!("{:}", calibration_values.iter().sum::<i32>());

    Ok(())
}
