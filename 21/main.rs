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

const NTILES: usize = 9;

fn move_in_dir(
    garden: &Garden,
    from: &(i32, i32),
    dir: usize
) -> Option<(i32, i32)> {
    let dp = DIRS[dir];
    let nx = from.0 as i32 + dp.0;
    let ny = from.1 as i32 + dp.1;

    if nx < garden.x_bounds.0 || nx > garden.x_bounds.1 ||
        ny < garden.y_bounds.0 || ny > garden.y_bounds.1 {
        return None;
    }

    let c = garden.lookup_infinite(&(nx, ny));

    if c == '#' {
        return None;
    }

    return Some((nx, ny));
}

fn build_min_distance(
    garden: &Garden,
    start: &(i32, i32),
) -> BTreeMap<(i32, i32), u32> {
    // x, y, dir, straight
    let mut min_distance: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut frontier = BinaryHeap::new();

    for dir in 0..4 {
        if let Some(dp) = move_in_dir(garden, start, dir) {
            frontier.push((dp.0, dp.1, 1));
        }
    }

    while let Some((x, y, distance)) = frontier.pop() {
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

struct Garden {
    grid: Vec<Vec<char>>,
    origin: (usize, usize),
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
}

impl Garden {
    fn new(grid: Vec<Vec<char>>, origin: (usize, usize)) -> Garden {
        let cols = grid[0].len();
        let rows = grid.len();

        let total_x = cols * NTILES;
        let total_y = rows * NTILES;

        let minx = -((total_x / 2) as i32);
        let maxx = (total_x / 2) as i32;
        let miny = -((total_y / 2) as i32);
        let maxy = (total_y / 2) as i32;

        return Garden{
            grid: grid,
            origin: origin,
            x_bounds: (minx, maxx),
            y_bounds: (miny, maxy),
        };
    }

    fn lookup_infinite(&self, coord: &(i32, i32)) -> char {
        let offset = (coord.0 + self.origin.0 as i32, coord.1 + self.origin.1 as i32);
        let rem = (
            offset.0.rem_euclid(self.grid[0].len() as i32),
            offset.1.rem_euclid(self.grid.len() as i32),
        );

        return self.grid[rem.1 as usize][rem.0 as usize];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
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
        grid.push(row);
    }

    let garden = Garden::new(grid, start);

    let min_distance = build_min_distance(&garden, &(0, 0));

    let n = (NTILES / 2) as i32;

    for y in garden.y_bounds.0..=garden.y_bounds.1 {
        if (y + garden.origin.1 as i32) % (garden.grid[0].len() as i32) == 0 {
            println!("{}", "-".repeat(((garden.x_bounds.1 - garden.x_bounds.0 + 1) as usize + NTILES - 1) * 5));
        }
        for x in garden.x_bounds.0..=garden.x_bounds.1 {
            if (x + garden.origin.0 as i32) % (garden.grid.len() as i32) == 0 {
                print!(" | ");
            }
            if let Some(distance) = min_distance.get(&(x, y)) {
                print!(" {:>3} ", distance);
            } else {
                print!(" ### ");
            }
        }
        println!("");
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

/* Thoughts:
 *
 * - We can probably work out how "wide" the solid area is
 *    num blocks: 26M - (block(1,0) top left) / block_width
 * - Then we can work out the top-left of that block
 *    - Need a function which we give block coords, and it gives distance to 4 corners(?)
 *    - Confirm that this block has corners <=goal and >=goal
 *      NOTE: Corners not enough to be sure it's complete. Row y=0 is "weird"
 * - Then we know there's a strip of blocks up to this of solid
 * - How many stragglers?
 *    - Need a function which fills out block, given corners
 *       - First fill edges based on relative position to origin?
 *       - Then fill from minimum coord?
 */
