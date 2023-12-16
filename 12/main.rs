use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn solve(
    pattern: &Vec<u8>,
    groups: &Vec<usize>,
    min_start: usize,
    i: usize,
    memo: &mut HashMap<(usize, usize), u64>) -> u64 {

    //let indent = "  ".repeat(i + 1);
    //println!("{indent} {i}, {groups:?} {min_start}");

    if let Some(solutions) = memo.get(&(min_start, i)) {
        //println!("{indent} <-- memo {solutions}");
        return *solutions;
    }

    let sum_after = groups[i + 1..].iter().sum::<usize>();
    let mut max_start = pattern.len() - sum_after - (groups.len() - i - 1) - groups[i];

    // Can't skip the next hash
    if let Some(next_hash) = pattern[min_start..].iter().position(|&b| b == b'#') {
        let next_hash = next_hash + min_start;
        //println!("{indent} next_hash: {next_hash}");
        max_start = std::cmp::min(max_start, next_hash);
    }

    let mut solutions = 0;

    for bit in min_start..=max_start {
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
            //println!("recurse {}", i + 1);
            solutions += solve(pattern, groups, end + 1, i + 1, memo);
        } else {
            // Special case for the last group
            if let Some(last_hash) = pattern.iter().rposition(|x| *x == b'#') {
                if last_hash > end {
                    //println!("hash after {}", bit);
                    continue;
                }
            }

            //println!("OK");
            solutions += 1;
        }
    }

    memo.insert((min_start, i), solutions);
    //println!("{} <-- {}", indent, solutions);
    return solutions;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    let mut total2 = 0;

    for line in reader.lines() {
        let line = line?;

        let (pattern, groups) = line.split_once(" ").unwrap();

        let groups: Vec<_> = groups.split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        //println!("pattern: {}, groups: {:?}", pattern, groups);

        let mut memo = HashMap::new();
        let pattern = pattern.bytes().collect();
        let solutions = solve(&pattern, &groups, 0, 0, &mut memo);

        //println!("solutions: {}", solutions);

        total += solutions;

        let mut pattern2 = Vec::new();
        for repeat in 0..5 {
            for b in &pattern {
                pattern2.push(*b);
            }
            if repeat < 4 {
                pattern2.push(b'?');
            }
        }
        let groups2 = groups.repeat(5);

        //println!("pattern2: {}, groups2: {:?}", String::from_utf8_lossy(&pattern2), groups2);

        let mut memo2 = HashMap::new();
        let solutions2 = solve(&pattern2, &groups2, 0, 0, &mut memo2);
        total2 += solutions2;
    }

    println!("{total}");
    println!("{total2}");

    Ok(())
}
