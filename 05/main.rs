use std::collections::HashMap;
use std::env;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

#[derive(Debug)]
struct MapRange {
    from: Range<u64>,
    to: u64,
}

#[derive(Debug)]
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

fn range_look_up_in(map: &Map, range: &Range::<u64>) -> Vec<Range<u64>> {
    let mut out = Vec::new();

    let mut remainder = Range{ start: range.start, end: range.end };

    for range in map.ranges.iter() {
        if remainder.start < range.from.start {
            let start_before = remainder.start;
            let end_before = min(remainder.end, range.from.start);

            // 1:1 mapping, because this is outside the range
            out.push(Range{
                start: start_before,
                end: end_before,
            });

            remainder = Range{
                start: end_before,
                end: remainder.end,
            }
        }

        let start_overlap = remainder.start;
        let end_overlap = min(remainder.end, range.from.end);
        if start_overlap < end_overlap {
            let diff = end_overlap - start_overlap;
            let offset = start_overlap - range.from.start;
            let start = range.to + offset;
            out.push(Range{
                start: start,
                end: start + diff,
            });

            remainder = Range{
                start: end_overlap,
                end: remainder.end,
            }
        }

        if !(remainder.end > remainder.start) {
            break;
        }
    }

    if remainder.start < remainder.end {
        // 1:1 mapping, because this is outside the range
        out.push(Range{
            start: remainder.start,
            end: remainder.end,
        });
    }

    out.sort_by_key(|v| v.start);

    return out;
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

                    let mr = MapRange{
                        from: Range{ start: parts[1], end: parts[1] + parts[2] },
                        to: parts[0],
                    };

                    let pos = ranges.binary_search_by_key(&mr.from.start, |v| v.from.start)
                        .unwrap_or_else(|e| e);
                    ranges.insert(pos, mr);
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
    let mut entries = seeds.clone();
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

    let mut seed_ranges: Vec<Range<u64>> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let size = seeds[i + 1];
        seed_ranges.push(Range{
            start: start,
            end: start + size,
        });
    }

    let mut from = String::from("seed");
    let mut ranges = seed_ranges.clone();
    loop {
        match maps.get(&from) {
            Some(m) => {
                let to = &m.to;

                let mut next_ranges: Vec<Range<u64>> = Vec::new();
                for range in ranges.iter() {
                    let mut this_ranges = range_look_up_in(m, range);

                    next_ranges.append(&mut this_ranges);
                }

                ranges = next_ranges;

                // I don't know how to manage the lifetime properly, so just
                // copy
                from = to.to_string();
            },
            None => { break; },
        }
    }

    ranges.sort_by_key(|r| r.start);

    println!("{}", ranges[0].start);

    Ok(())
}
