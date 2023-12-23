use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
use std::fs::File;

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn move_in_dir(
    map: &Vec<Vec<char>>,
    from: &(usize, usize),
    dir: usize
) -> Option<(usize, usize)> {
    let dp = DIRS[dir];
    let nx = from.0 as i32 + dp.0;
    let ny = from.1 as i32 + dp.1;

    if nx < 0 || nx >= map[0].len() as i32 ||
        ny < 0 || ny >= map.len() as i32 {
        return None;
    }

    if map[ny as usize][nx as usize] == '#' {
        return None;
    }

    return Some((nx as usize, ny as usize));
}

fn explore(
    map: &Vec<Vec<char>>,
    from: &(usize, usize),
    goal: &(usize, usize),
    path: &HashSet<(usize, usize)>,
    prev_cell: char,
    part2: &bool) -> Vec<usize> {

    let mut current = *from;
    let mut path = path.clone();
    let start_n = path.len();

    loop {
        path.insert(current.clone());

        if current == *goal {
            return vec![path.len() - 1];
        }

        let cell = map[current.1][current.0];
        let dirs = match (part2, cell) {
            (false, '>') => 0..=0,
            (false, 'v') => 1..=1,
            (false, '<') => 2..=2,
            (false, '^') => 3..=3,
            (false, _) => 0..=3,
            (true, _) => 0..=3,
        };

        let dirs: Vec<usize> = dirs.collect();

        let options: Vec<(usize, usize)> = dirs.iter()
            .filter_map(
                |d| move_in_dir(map, &current, *d)
            )
            .filter(|v| !path.contains(v))
            .collect();

        if options.len() > 1 {
            let mut results = Vec::new();
            for option in options {
                let lengths = explore(map, &option, goal, &path, prev_cell, part2);
                results.extend(lengths);
            }
            return results;
        } else if options.len() == 0 {
            return vec![];
        } else {
            current = options[0];
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row = Vec::from_iter(line.chars());
        map.push(row);
    }

    let mut start_x = 0;
    for (i, x) in map[0].iter().enumerate() {
        if *x == '.' {
            start_x = i;
        }
    }
    let start = (start_x, 0);

    let mut end_x = 0;
    for (i, x) in map[map.len() - 1].iter().enumerate() {
        if *x == '.' {
            end_x = i;
        }
    }
    let end = (end_x, map.len() - 1);

    let paths = explore(&map, &start, &end, &HashSet::new(), '.', &false);
    println!("{:?}", paths.iter().max().unwrap());

    let paths = explore(&map, &start, &end, &HashSet::new(), '.', &true);
    println!("{:?}", paths.iter().max().unwrap());

    Ok(())
}
