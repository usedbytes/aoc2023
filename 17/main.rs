use std::env;
use std::fs::File;
use std::collections::BTreeMap;
use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn move_in_dir(
    map: &Vec<Vec<u8>>,
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

    return Some((nx as usize, ny as usize));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    cost: u32,
    dir: usize,
    straight: usize,
}

// This is materially the same as the binary_heap example in the docs
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn allowed_dirs_part1(straight: usize) -> (bool, bool) {
    return (true, straight < 2);
}

fn allowed_dirs_part2(straight: usize) -> (bool, bool) {
    return (straight >= 3, straight < 9);
}

fn explore(
    map: &Vec<Vec<u8>>,
    allowed_dirs: fn(usize) -> (bool, bool),
) -> u32 {
    let mut to_explore = BinaryHeap::new();

    // x, y, dir, straight
    let mut cheapest: BTreeMap<(usize, usize, usize, usize), u32> = BTreeMap::new();

    let goal = (map[0].len() - 1, map.len() - 1);

    // Need to seed the search with both East and South
    to_explore.push(State{
        pos: (0, 0),
        cost: 0,
        dir: 0,
        straight: 0,
    });

    to_explore.push(State{
        pos: (0, 0),
        cost: 0,
        dir: 1,
        straight: 0,
    });

    while let Some(State{ pos, cost, dir, straight }) = to_explore.pop() {

        //println!("{:?}, {}, {}, {}", pos, cost, dir, straight);

        if let Some(current_cost) = cheapest.get(&(pos.0, pos.1, dir, straight)) {
            if cost >= *current_cost {
                //println!("Already cheaper: {}", current_cost);
                continue;
            }
        }

        cheapest.insert((pos.0, pos.1, dir, straight), cost);

        let (allowed_turn, allowed_straight) = allowed_dirs(straight);

        if pos == goal && allowed_turn {
            return cost;
        }

        if allowed_turn { // Turn right
            let nd = ((dir as i32 + 1).rem_euclid(4)) as usize;
            if let Some(next) = move_in_dir(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("right next: {:?}, {}", next, next_cost);
                to_explore.push(State{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: 0,
                });
            }
        }

        if allowed_turn { // Turn left
            let nd = ((dir as i32 - 1).rem_euclid(4)) as usize;
            if let Some(next) = move_in_dir(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("left next: {:?}, {}", next, next_cost);
                to_explore.push(State{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: 0,
                });
            }
        }

        if allowed_straight { // Straight
            let nd = dir;
            if let Some(next) = move_in_dir(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("straight next: {:?}, {}", next, next_cost);
                to_explore.push(State{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: straight + 1,
                })
            }
        }
    }

    unreachable!();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = Vec::from_iter(
            line.chars()
                .map(|v| v.to_digit(10).unwrap() as u8)
        );
        map.push(row);
    }

    let cost = explore(&map, allowed_dirs_part1);
    println!("{cost}");

    let cost2 = explore(&map, allowed_dirs_part2);
    println!("{cost2}");

    Ok(())
}
