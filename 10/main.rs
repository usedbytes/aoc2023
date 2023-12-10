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

#[derive(Debug)]
struct Pipe {
    ends: [Direction; 2],
    left_side: Vec<(i32, i32)>, // If entering from ends[0]
    right_side: Vec<(i32, i32)>,
}

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
) -> (Vec<(i32, i32)>, BTreeSet<(i32, i32)>, BTreeSet<(i32, i32)>) {
    let move_dirs = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    let mut path = Vec::new();

    let mut side_a = BTreeSet::new();
    let mut side_b = BTreeSet::new();

    let mut current_pos = *start;
    let mut pipe = map.get(&current_pos).unwrap();
    let mut going = pipe.ends[1];
    loop {
        path.push(current_pos);
        if going == pipe.ends[1] {
            for p in &pipe.left_side {
                side_a.insert(*p);
            }
            for p in &pipe.right_side {
                side_b.insert(*p);
            }
        } else {
            for p in &pipe.left_side {
                side_b.insert(*p);
            }
            for p in &pipe.right_side {
                side_a.insert(*p);
            }
        }

        let delta = move_dirs.get(&going).unwrap();
        let next_pos = (current_pos.0 + delta.0, current_pos.1 + delta.1);

        pipe = map.get(&next_pos).unwrap();

        let coming = flip(&going);
        if coming == pipe.ends[0] {
            going = pipe.ends[1];
        } else {
            going = pipe.ends[0];
        }

        if next_pos == *start {
            break;
        }

        current_pos = next_pos;
    }

    return (path, side_a, side_b);
}


fn find_inside(
    map: &HashMap<(i32, i32), Pipe>,
    path: &Vec<(i32, i32)>,
    size: &(i32, i32),
    path_side_a: &BTreeSet<(i32, i32)>,
    path_side_b: &BTreeSet<(i32, i32)>,
    print: bool,
) -> usize {
    let mut to_search: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut path_set = BTreeSet::from_iter(path.iter());

    let move_dirs = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    let mut side_a: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut side_a_oob = 0;
    for p in path_side_a {
        if path_set.contains(p) {
            continue;
        } else if (p.0 < 0) || (p.1 < 0) ||
            (p.0 > size.0 - 1) || (p.1 > size.1 - 1) {
            side_a_oob += 1;
            continue;
        }
        to_search.insert(*p);
        side_a.insert(*p);
    }

    while to_search.len() > 0 {
        let current = to_search.pop_first().unwrap();
        side_a.insert(current);
        for (dir, delta) in &move_dirs {
            let check = (current.0 + delta.0, current.1 + delta.1);
            if (check.0 < 0) || (check.1 < 0) ||
                (check.0 > size.0 - 1) || (check.1 > size.1 - 1) {
                side_a_oob += 1;
                continue;
            } else if path_set.contains(&check) {
                continue;
            } else if !side_a.contains(&check) {
                to_search.insert(check);
            }
        }
    }

    let mut side_b: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut side_b_oob = 0;
    for p in path_side_b {
        if path_set.contains(p) {
            continue;
        } else if (p.0 < 0) || (p.1 < 0) ||
            (p.0 > size.0 - 1) || (p.1 > size.1 - 1) {
            side_b_oob += 1;
            continue;
        }
        to_search.insert(*p);
        side_b.insert(*p);
    }

    while to_search.len() > 0 {
        let current = to_search.pop_first().unwrap();
        for (dir, delta) in &move_dirs {
            let check = (current.0 + delta.0, current.1 + delta.1);
            if (check.0 < 0) || (check.1 < 0) ||
                (check.0 > size.0 - 1) || (check.1 > size.1 - 1) {
                side_b_oob += 1;
                continue;
            } else if path_set.contains(&check) {
                continue;
            } else if !side_b.contains(&check) {
                to_search.insert(check);
            }
            side_b.insert(check);
        }
    }

    assert!(side_a_oob == 0 || side_b_oob == 0);
    assert!((path_set.len() + side_a.len() + side_b.len()) as i32 ==
            size.0 * size.1);

    if print {
        for row in 0..size.1 {
            for col in 0..size.0 {
                let cell = &(col as i32, row as i32);
                if path_set.contains(&cell) {
                    print!("p");
                } else if side_a.contains(&cell) {
                    print!("A");
                } else if side_b.contains(&cell) {
                    print!("B");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    if side_a_oob == 0 {
        return side_a.len();
    } else {
        return side_b.len();
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
let file = File::open(fname)?; let reader = io::BufReader::new(file);
    let pipe_bases: HashMap<char, ([Direction; 2], Vec<(i32, i32)>, Vec<(i32, i32)>)> = HashMap::from([
        (
            '|',
            (
                [Direction::South, Direction::North],
                vec![(-1, 0)],
                vec![(1, 0)],
            ),
        ),
        (
            '-',
            (
                [Direction::West, Direction::East],
                vec![(0, -1)],
                vec![(0, 1)],
            ),
        ),
        (
            '7',
            (
                [Direction::West, Direction::South],
                vec![(1, 0), (0, -1)],
                vec![],
            ),
        ),
        (
            'J',
            (
                [Direction::North, Direction::West],
                vec![(1, 0), (0, 1)],
                vec![],
            ),
        ),
        (
            'L',
            (
                [Direction::East, Direction::North],
                vec![(-1, 0), (0, 1)],
                vec![],
            ),
        ),
        (
            'F',
            (
                [Direction::South, Direction::East],
                vec![(-1, 0), (0, -1)],
                vec![],
            ),
        ),
    ]);

    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut size = (0, 0);

    for (row, l) in reader.lines().enumerate() {
        let line = l?;
        for (col, letter) in line.chars().enumerate() {
            if let Some(base) = pipe_bases.get(&letter) {
                let ends = &base.0;

                let cell = (col as i32, row as i32);

                let left = base.1
                    .iter()
                    .map(|d| (cell.0 + d.0, cell.1 + d.1))
                    .collect();
                let right = base.2
                    .iter()
                    .map(|d| (cell.0 + d.0, cell.1 + d.1))
                    .collect();
                map.insert(
                    cell,
                    Pipe{
                        ends: ends.clone(),
                        left_side: left,
                        right_side: right,
                    }
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
            if let Some(idx) = pipe.ends.iter().position(|v| *v == entry) {
                start_ends.push(dir);
            }
        }
    }

    assert!(start_ends.len() == 2);

    for (c, base) in pipe_bases.iter() {
        let ends = &base.0;
        if start_ends.contains(&ends[0]) && start_ends.contains(&ends[1]) {
            let cell = start;
            let left = base.1
                .iter()
                .map(|d| (cell.0 + d.0, cell.1 + d.1))
                .collect();
            let right = base.2
                .iter()
                .map(|d| (cell.0 + d.0, cell.1 + d.1))
                .collect();
            map.insert(
                cell,
                Pipe{
                    ends: ends.clone(),
                    left_side: left,
                    right_side: right,
                }
            );
        }
    }

    let (path, side_a, side_b) = trace_path(&map, &start);
    println!("{}", (path.len() + 1) / 2);

    let inside = find_inside(&map, &path, &size, &side_a, &side_b, false);
    println!("{}", inside);

    Ok(())
}