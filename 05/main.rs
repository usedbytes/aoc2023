use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

struct MapRange {
    from: Range<u64>,
    to: u64,
}

struct Map {
    to: String,
    ranges: Vec<MapRange>,
}

fn look_up_in(map: &Map, v: &u64) -> u64 {
    for range in &map.ranges {
        if range.from.contains(&v) {
            let out = range.to + (v - range.from.start);
            return out;
        }
    }

    return *v;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let line = lines.next().unwrap()?;

    let (_, rest) = line.split_once(": ").unwrap();
    let seeds: Vec<u64> = rest.split_whitespace()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect();

    _ = lines.next();

    let mut maps = HashMap::new();

    let mut done = false;
    while !done {
        let line = lines.next().unwrap()?;
        let (mapping, _) = line.split_once(" ").unwrap();
        let (from, to) = mapping.split_once("-to-").unwrap();

        let mut map = Map{
            to: to.to_string(),
            ranges: Vec::new(),
        };
        let ranges = &mut map.ranges;

        'map: loop {
            match lines.next() {
                Some(line) => {
                    let line = line?;
                    if line.len() == 0 {
                        break 'map;
                    }

                    let parts: Vec<u64> = line.split_whitespace()
                        .map(|v| v.trim().parse::<u64>().unwrap())
                        .collect();

                    ranges.push(MapRange{
                        from: Range{ start: parts[1], end: parts[1] + parts[2] },
                        to: parts[0],
                    });
                },
                None => {
                    done = true;
                    break 'map;
                },
            }
        }

        maps.insert(from.to_string(), map);
    }

    let mut from = String::from("seed");
    let mut entries = seeds;
    loop {
        match maps.get(&from) {
            Some(m) => {
                let to = &m.to;
                entries = entries.iter()
                    .map(|v| look_up_in(m, v))
                    .collect();
                // I don't know how to manage the lifetime properly, so just
                // copy
                from = to.to_string();
            },
            None => { break; },
        }
    }

    let min = entries.iter().min().unwrap();
    println!("{}", min);

    Ok(())
}
