use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn calc_diff(vals: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 1..vals.len() {
        result.push(vals[i] - vals[i-1]);
    }

    return result;
}

fn extrapolate(vals: &Vec<i32>, start: bool) -> (i32, i32) {
    let diff = calc_diff(vals);
    if diff.iter().all(|v| *v == 0) {
        // Doesn't actually matter which we return
        return (vals[0], vals[vals.len()-1]);
    }

    let (ex_start, ex_end) = extrapolate(&diff, start);

    return (vals[0] - ex_start, vals[vals.len()-1] + ex_end);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let vals = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        let (start, end) = extrapolate(&vals, false);
        part1 += end;
        part2 += start;
    }

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
