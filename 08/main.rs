use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let route = lines.next().unwrap().unwrap().into_bytes();

    lines.next();

    let mut map = HashMap::new();

    let line_re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    for line in lines {
        let line = line.unwrap();

        let caps = line_re.captures(&line).unwrap();

        map.insert(
            String::from(caps.get(1).unwrap().as_str()),
            (
                String::from(caps.get(2).unwrap().as_str()),
                String::from(caps.get(3).unwrap().as_str())
            )
        );
    }

    let mut node = "AAA";
    let mut moves = 0;
    'done: loop {
        for i in 0..route.len() {
            let options = map.get(node).unwrap();
            match &route[i] {
                b'L' => {
                    node = &options.0;
                },
                b'R' => {
                    node = &options.1
                },
                _ => { panic!(); },
            }

            moves += 1;

            if node == "ZZZ" {
                break 'done;
            }
        }
    }

    println!("{}", moves);

    Ok(())
}
