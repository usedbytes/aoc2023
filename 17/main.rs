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

fn get_next(
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

fn explore(
    map: &Vec<Vec<u8>>,
    cheapest: &mut BTreeMap::<(usize, usize), u32>,
    from: &(usize, usize),
    cost: u32,
    dir: usize,
    straight: u32,
    depth: usize) {

    let indent = "   ".repeat(depth);
    println!("{indent} {:?}, {}, {}, {}", from, straight, dir, cost);

    if let Some(current_cost) = cheapest.get_mut(from) {
        if cost >= *current_cost {
            println!("Already cheaper: {}", current_cost);
            return;
        }
    }

    cheapest.insert(*from, cost);

    { // Turn right
        let nd = ((dir as i32 + 1).rem_euclid(4)) as usize;
        println!("nd: {nd}");
        if let Some(next) = get_next(map, from, nd) {
            let next_cost = cost + map[from.1][from.0] as u32;
            explore(map, cheapest, &next, next_cost, nd, 0, depth + 1);
        }
    }

    { // Turn left
        let nd = ((dir as i32 - 1).rem_euclid(4)) as usize;
        println!("nd: {nd}");
        if let Some(next) = get_next(map, from, nd) {
            let next_cost = cost + map[from.1][from.0] as u32;
            explore(map, cheapest, &next, next_cost, nd, 0, depth + 1);
        }
    }

    if straight < 2 { // Turn left
        let nd = dir;
        if let Some(next) = get_next(map, from, nd) {
            let next_cost = cost + map[from.1][from.0] as u32;
            explore(map, cheapest, &next, next_cost, nd, straight + 1, depth + 1);
        }
    }
}

// (x, y), cost, dir, straight
type State = ((usize, usize), u32, usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State2 {
    pos: (usize, usize),
    cost: u32,
    dir: usize,
    straight: usize,
}

// This is materially the same as the binary_heap example in the docs
impl Ord for State2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| other.straight.cmp(&self.straight))
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn explore2(
    map: &Vec<Vec<u8>>,
    cheapest: &mut BTreeMap::<(usize, usize, usize), u32>,
) {
    //let mut to_explore: Vec<State> = Vec::new();
    let mut to_explore = BinaryHeap::new();

    let first_state = State2{
        pos: (0, 0),
        cost: 0,
        dir: 0,
        straight: 0
    };
    to_explore.push(first_state);

    let goal = (map[0].len() - 1, map.len() - 1);

    while let Some(State2{ pos, cost, dir, straight }) = to_explore.pop() {

        //println!("{:?}, {}, {}, {}", pos, cost, dir, straight);

        if let Some(current_cost) = cheapest.get(&(pos.0, pos.1, straight)) {
            if cost >= *current_cost {
                //println!("Already cheaper: {}", current_cost);
                continue;
            }
        }

        cheapest.insert((pos.0, pos.1, straight), cost);

        if pos == goal {
            println!("goal in {}", cost);
        }

        { // Turn right
            let nd = ((dir as i32 + 1).rem_euclid(4)) as usize;
            if let Some(next) = get_next(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("right next: {:?}, {}", next, next_cost);
                to_explore.push(State2{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: 0
                });
            }
        }

        { // Turn left
            let nd = ((dir as i32 - 1).rem_euclid(4)) as usize;
            if let Some(next) = get_next(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("left next: {:?}, {}", next, next_cost);
                to_explore.push(State2{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: 0
                });
            }
        }

        if straight < 2 { // Straight
            let nd = dir;
            if let Some(next) = get_next(map, &pos, nd) {
                let next_cost = cost + map[next.1][next.0] as u32;
                //println!("straight next: {:?}, {}", next, next_cost);
                //to_explore.push((next, next_cost, nd, straight + 1));
                to_explore.push(State2{
                    pos: next,
                    cost: next_cost,
                    dir: nd,
                    straight: straight + 1,
                })
            }
        }
    }
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

    println!("{:?}", map);

    let mut cheapest: BTreeMap<(usize, usize, usize), u32> = BTreeMap::new();

    //explore(&map, &mut hottest, &(0, 0), 0, 0, 0, 0);
    explore2(&map, &mut cheapest);

    println!("{:?}", cheapest.entry((map.len() - 1, map[0].len() - 1, 0)));
    println!("{:?}", cheapest.entry((map.len() - 1, map[0].len() - 1, 1)));
    println!("{:?}", cheapest.entry((map.len() - 1, map[0].len() - 1, 2)));
    println!("{:?}", cheapest.entry((map.len() - 1, map[0].len() - 1, 3)));

    Ok(())
}
