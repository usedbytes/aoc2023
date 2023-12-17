use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn checksum(s: &str) -> u8 {
    let mut val: u32 = 0;
    for b in s.bytes() {
        val += b as u32;
        val *= 17;
        val %= 256;
    }

    return val as u8;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        for insn in line.split(",") {
            total += checksum(insn) as u32;
        }
    }

    println!("{total}");

    Ok(())
}
