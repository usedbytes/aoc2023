use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::collections::{BTreeMap, BinaryHeap};
use std::fs::File;

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

const NTILES: usize = 3;

fn move_in_dir(
    map: &Vec<Vec<char>>,
    from: &(usize, usize),
    dir: usize
) -> Option<(usize, usize)> {
    let dp = DIRS[dir];
    let nx = from.0 as i32 + dp.0;
    let ny = from.1 as i32 + dp.1;

    let maxx = map[0].len() * NTILES;
    let maxy = map.len() * NTILES;

    if nx < 0 || nx >= maxx as i32 ||
        ny < 0 || ny >= maxy as i32 {
        return None;
    }

    if map[ny as usize % map.len()][nx as usize % map[0].len()] == '#' {
        return None;
    }

    return Some((nx as usize, ny as usize));
}

fn build_min_distance(
    garden: &Vec<Vec<char>>,
    start: &(usize, usize),
) -> BTreeMap<(usize, usize), u32> {
    // x, y, dir, straight
    let mut min_distance: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    let mut frontier = BinaryHeap::new();

    for dir in 0..4 {
        if let Some(dp) = move_in_dir(garden, start, dir) {
            frontier.push((dp.0, dp.1, 1));
        }
    }

    while let Some((x, y, distance)) = frontier.pop() {
        //println!("({}, {}), {}", x, y, distance);

        if let Some(min) = min_distance.get(&(x, y)) {
            if distance >= *min {
                continue;
            }
        }

        min_distance.insert((x, y), distance);

        for dir in 0..4 {
            if let Some(np) = move_in_dir(garden, &(x, y), dir) {
                frontier.push((np.0, np.1, distance + 1));
            }
        }
    }

    return min_distance;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut garden = Vec::new();
    let mut start = (0, 0);

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        let mut row: Vec<char> = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x, y);
            }
        }
        garden.push(row);
    }

    let min_distance = build_min_distance(&garden, &start);

    for y in 0..garden.len() * NTILES {
        if y % garden.len() == 0 {
            println!("     {}\n", "-".repeat((garden[0].len() * NTILES + 1) * 5));
        }
        for x in 0..garden[0].len() * NTILES {
            if x % garden[0].len() == 0 {
                print!("  |  ");
            }
            if let Some(distance) = min_distance.get(&(x, y)) {
                if (*distance & 1 == 0) {
                    print!(" {:>3} ", distance);
                } else {
                    print!("     ");
                }
                //print!(" {:>3} ", distance);
            } else {
                print!(" ### ");
            }
        }
        println!("\n");
    }

    // A square is reachable if its min distance is less than the number of
    // steps, and also has the same "LSB" as the number of steps.
    // For example, with an "even" number of steps, we can't reach any tiles
    // with "odd" distances, because we'd need to take two detours to get to
    // them.
    let n: u32 = 64;
    let mut reachable = 0;
    for v in min_distance.values() {
        if *v <= n && (*v & 1) == (n & 1) {
            reachable += 1;
        }
    }

    println!("{}", reachable);

    Ok(())
}
