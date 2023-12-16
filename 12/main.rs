use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

struct Mask {
    min: usize,
    max: usize,
    mask: u32,
}

impl Mask {
    fn new(min: usize, max: usize) -> Mask {
        let n = (max + 1) - min;
        let mask = (1 << n) - 1;
        return Mask{
            min: min,
            max: max,
            mask: mask << min,
        };
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:020b}", self.mask)
    }
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn solve(
    pattern: &Vec<u8>,
    groups: &Vec<usize>,
    start_masks: &Vec<Mask>,
    start_mask: &Mask,
    i: usize) -> u32 {

    //let indent = "  ".repeat(i + 1);
    //println!("{indent} {i}, {groups:?} {start_mask}");

    let top_bit = start_mask.max;
    let bottom_bit = start_mask.min;
    let mut solutions = 0;

    for bit in bottom_bit..=top_bit {
        //print!("{indent} bit {bit}: ");
        // Can it start here?
        if pattern[bit] == b'.' {
            //println!("bad start (.)");
            continue;
        } else if bit > 0 && pattern[bit - 1] == b'#' {
            //println!("bad start (#)");
            continue;
        }

        // Can it end here?
        let end = bit + groups[i];
        if end < pattern.len() && pattern[end] == b'#' {
            //println!("bad end");
            continue;
        }

        // Is the middle OK?
        let mut ok = true;
        for j in bit..bit + groups[i] {
            if pattern[j] == b'.' {
                //println!("bad middle {}", j);
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }

        if i < groups.len() - 1 {
            let next_start = std::cmp::max(start_masks[i].min, bit + groups[i] + 1);

            // Make sure we don't "jump over" a hash
            let mut next_end = start_masks[i + 1].max;
            for j in end..pattern.len() {
                if pattern[j] == b'#' {
                    next_end = std::cmp::min(next_end, j);
                }
            }

            // Sanity
            if next_start > next_end {
                panic!();
            }

            let next_mask = Mask::new(next_start, next_end);
            //println!("recurse {}", i + 1);
            solutions += solve(pattern, groups, start_masks, &next_mask, i + 1);
        } else {
            // Special case for the last group
            if let Some(last_hash) = pattern.iter().rposition(|x| *x == b'#') {
                if start_mask.min <= last_hash && bit > last_hash {
                    //println!("hash before {}", bit);
                    continue;
                } else if last_hash > end {
                    //println!("hash after {}", bit);
                    continue;
                }
            }

            //println!("OK");
            solutions += 1;
        }
    }

    //println!("{} <-- {}", indent, solutions);
    return solutions;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let (pattern, groups) = line.split_once(" ").unwrap();

        let groups: Vec<_> = groups.split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let mut min_starts = vec![0; groups.len()];
        for i in 1..min_starts.len() {
            min_starts[i] = min_starts[i - 1] + groups[i - 1] + 1;
        }

        let mut max_starts = vec![0; groups.len()];
        max_starts[groups.len() - 1] = pattern.len() - groups[groups.len() - 1];
        for i in (0..max_starts.len() - 1).rev() {
            max_starts[i] = max_starts[i + 1] - groups[i] - 1;
        }

        if let Some(first_hash) = pattern.find('#') {
            max_starts[0] = std::cmp::min(first_hash, max_starts[0]);
        }

        let start_masks = Vec::from_iter(
            (0..groups.len())
            .map(|i| Mask::new(
                min_starts[i],
                max_starts[i]
            ))
        );

        //println!("pattern: {}, groups: {:?}, starts: {:?}", pattern, groups, start_masks);

        let pattern = pattern.bytes().collect();
        let solutions = solve(&pattern, &groups, &start_masks, &start_masks[0], 0);
        //println!("solutions: {}", solutions);

        total += solutions;
    }

    println!("{total}");

    Ok(())
}
