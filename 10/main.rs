use std::collections::{HashMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[repr(u8)]
enum Direction {
    North = b'N',
    East = b'E',
    South = b'S',
    West = b'W',
}

type Pipe = [Direction; 2];

fn flip(d: &Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn trace_path(
    map: &HashMap<(i32, i32), Pipe>,
    start: &(i32, i32)
) -> Vec<(i32, i32)> {
    let move_dirs = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    let mut path = Vec::new();

    let mut current_pos = *start;
    let mut pipe = map.get(&current_pos).unwrap();
    let mut going = pipe[1];
    loop {
        path.push(current_pos);
        let delta = move_dirs.get(&going).unwrap();
        let next_pos = (current_pos.0 + delta.0, current_pos.1 + delta.1);

        pipe = map.get(&next_pos).unwrap();

        let coming = flip(&going);
        if coming == pipe[0] {
            going = pipe[1];
        } else {
            going = pipe[0];
        }

        if next_pos == *start {
            break;
        }

        current_pos = next_pos;
    }

    return path;
}

fn is_inside(
    path: &BTreeSet<&(i32, i32)>,
    map: &HashMap<(i32, i32), Pipe>,
    cell: &(i32, i32)
) -> bool {
    let row = cell.1;
    let col = cell.0;

    let mut crossings = 0;
    for i in (0..col).rev() {
        let check = (i, row);
        if let Some(_) = path.get(&check) {
            let mp = map.get(&check).unwrap();
            if mp.contains(&Direction::North) {
                crossings += 1;
            }
        }
    }

    return (crossings & 1) == 1;
}

fn find_inside(
    path: &Vec<(i32, i32)>,
    map: &HashMap<(i32, i32), Pipe>,
    size: &(i32, i32),
) -> usize {
    let path_set = BTreeSet::from_iter(path.iter());
    let mut outside: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut inside: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut to_search: BTreeSet<(i32, i32)> = BTreeSet::new();

    for p in &path_set {
        to_search.insert(**p);
    }

    let move_dirs = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    while to_search.len() > 0 {
        let current = to_search.pop_first().unwrap();
        for (_, delta) in &move_dirs {
            let check = (current.0 + delta.0, current.1 + delta.1);
            if (check.0 < 0) || (check.1 < 0) ||
                (check.0 > size.0 - 1) || (check.1 > size.1 - 1) {
                // Out of bounds, do nothing with 'check'
                continue;
            } else if path_set.contains(&check) {
                // We already know 'check' is on path, do nothing
                continue;
            } else if outside.contains(&check) {
                if !path_set.contains(&current) {
                    outside.insert(current);
                }
            } else if inside.contains(&check) {
                if !path_set.contains(&current) {
                    inside.insert(current);
                }
            } else {
                to_search.insert(check);
            }
        }

        if !path_set.contains(&current) &&
                !outside.contains(&current) &&
                !inside.contains(&current) {
            if is_inside(&path_set, map, &current) {
                inside.insert(current);
            } else {
                outside.insert(current);
            }
        }
    }

    return inside.len();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let file = File::open(fname)?; let reader = io::BufReader::new(file);
    let pipe_ends: HashMap<char, [Direction; 2]> = HashMap::from([
        ('|', [Direction::South, Direction::North]),
        ('-', [Direction::West, Direction::East]),
        ('7', [Direction::West, Direction::South]),
        ('J', [Direction::North, Direction::West]),
        ('L', [Direction::East, Direction::North]),
        ('F', [Direction::South, Direction::East]),
    ]);

    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut size = (0, 0);

    for (row, l) in reader.lines().enumerate() {
        let line = l?;
        for (col, letter) in line.chars().enumerate() {
            if let Some(ends) = pipe_ends.get(&letter) {
                let cell = (col as i32, row as i32);
                map.insert(
                    cell,
                    ends.clone(),
                );
            } else if letter == 'S' {
                start = (col as i32, row as i32);
            }
        }

        size.0 = line.len() as i32;
        size.1 = (row + 1) as i32;
    }

    let move_dirs = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    let mut start_ends = Vec::new();

    for (dir, delta) in move_dirs {
        let check = (start.0 + delta.0, start.1 + delta.1);
        if let Some(pipe) = map.get(&check) {
            let entry = flip(&dir);
            if let Some(_) = pipe.iter().position(|v| *v == entry) {
                start_ends.push(dir);
            }
        }
    }

    assert!(start_ends.len() == 2);

    for (_, ends) in pipe_ends.iter() {
        if start_ends.contains(&ends[0]) && start_ends.contains(&ends[1]) {
            let cell = start;
            map.insert(
                cell,
                ends.clone(),
            );
        }
    }

    let path = trace_path(&map, &start);
    println!("{}", (path.len() + 1) / 2);

    let inside = find_inside(&path, &map, &size);
    println!("{}", inside);

    Ok(())
}
