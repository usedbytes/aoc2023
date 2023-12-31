use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn run_route<'a>(
    route: &Vec<u8>,
    map: &'a HashMap<String, (String, String)>,
    start: &str,
    goal_fn: fn(&String) -> bool
) -> (usize, &'a String) {
    let mut i = 0;

    let mut options = map.get(start).unwrap();
    loop {
        let d = &route[i % route.len()];
        let node = match d {
            b'L' => &options.0,
            b'R' => &options.1,
            _ => panic!(),
        };

        i += 1;

        if goal_fn(node) {
            return (i, node);
        }

        options = map.get(node).unwrap();
    }
}

fn get_factors(n: u32) -> HashSet<u32> {
    let mut i = 2;
    let mut factors = HashSet::new();

    while i <= n {
        let div = n / i;
        let rem = n % i;

        if rem == 0 {
            if !factors.insert(div) || !factors.insert(i) {
                return factors;
            }
        }

        i += 1;
    }

    return factors;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let route = lines.next().unwrap().unwrap().into_bytes();

    lines.next();

    let mut map = HashMap::new();
    let mut starts = Vec::new();

    let line_re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    for line in lines {
        let line = line.unwrap();

        let caps = line_re.captures(&line).unwrap();

        let node = caps.get(1).unwrap().as_str();
        if let Some(b'A') = node.as_bytes().last() {
            starts.push(String::from(node));
        }

        map.insert(
            String::from(caps.get(1).unwrap().as_str()),
            (
                String::from(caps.get(2).unwrap().as_str()),
                String::from(caps.get(3).unwrap().as_str())
            )
        );
    }

    // Run Part 1 - the "sample3" input doesn't have an AAA so just guard this
    // against that.
    if map.get("AAA").is_some() {
        let (moves, _) = run_route(&route, &map, "AAA", |n| n == "ZZZ");
        println!("Part 1: {:?}", moves);
    }

    // Match function for ending Part 2
    fn ends_with_z(s: &String) -> bool {
        return (&s).bytes().last().is_some_and(|b| b == b'Z');
    }

    // Find the route lengths for all starting positions.
    let mut route_lengths = Vec::new();
    for s in starts {
        let (moves, node) = run_route(&route, &map, &s, ends_with_z);
        let (moves2, node2) = run_route(&route, &map, node, ends_with_z);

        // First "lap" must be the same length and finish as the repeating lap,
        // otherwise CRT won't work (at least not without more work)
        assert!(moves == moves2);
        assert!(node == node2);

        route_lengths.push(moves);
    }

    // We need to find the least-common-multiple of all the routes, which we'll
    // do by factorising. Presumably the input is crafted to give prime factors
    let mut all_factors: HashSet<u32> = HashSet::new();
    for l in &route_lengths {
        let factors = get_factors(*l as u32);
        all_factors.extend(factors.iter());
    }

    // We must have all prime factors, otherwise more work is needed
    // to find lcm
    assert!(all_factors.len() == route_lengths.len() + 1);

    let mut product: u64 = 1;
    for f in &all_factors {
        product *= *f as u64;
    }

    println!("Part 2: {}", product);

    Ok(())
}
