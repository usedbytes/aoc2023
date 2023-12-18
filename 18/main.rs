use std::env;
use std::fs::File;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufRead};

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn flood_fill(
    perimeter: &BTreeMap<(i32, i32), u32>,
    start: &(i32, i32),
) -> usize {
    let mut trench = perimeter.clone();

    let mut search = Vec::new();
    search.push(*start);

    while let Some(pos) = search.pop() {
        trench.insert(pos, 0);

        for i in 0..4 {
            let dp = DIRS[i];
            let next_pos = (pos.0 + dp.0, pos.1 + dp.1);

            if let None = trench.get(&next_pos) {
                search.push(next_pos);
            }
        }
    }

    return trench.len();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let dig_dirs: HashMap<&str, (i32, i32)> = HashMap::from([
        ("U", (0, -1)),
        ("D", (0, 1)),
        ("L", (-1, 0)),
        ("R", (1, 0)),
    ]);

    let mut perimeter: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut pos: (i32, i32) = (0, 0);

    let mut max = (0, 0);
    let mut min = (0, 0);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split(" ").collect();

        let dp = dig_dirs.get(parts[0]).unwrap();
        let n = parts[1].parse::<i32>().unwrap();
        let color = u32::from_str_radix(&parts[2][2..8], 16).unwrap();

        for _ in 1..=n {
            pos.0 += dp.0;
            pos.1 += dp.1;
            perimeter.insert(pos, color);

            min.0 = std::cmp::min(min.0, pos.0);
            min.1 = std::cmp::min(min.1, pos.1);
            max.0 = std::cmp::max(max.0, pos.0);
            max.1 = std::cmp::max(max.1, pos.1);
        }
    }

    // FIXME: Assumes (1, 1) is inside, which might not be true.
    let start = (1, 1);

    let size = flood_fill(&perimeter, &start);
    println!("{size}");

    /*
    for row in min.1..=max.1 {
        for col in min.0..=max.0 {
            if let Some(_) = perimeter.get(&(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    */

    Ok(())
}
