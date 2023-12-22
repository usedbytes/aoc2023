use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct ParseErr;

impl Error for ParseErr {}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Vec3 {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        assert!(parts.len() == 3);

        let x = parts[0].parse::<u32>().or(Err(ParseErr))?;
        let y = parts[1].parse::<u32>().or(Err(ParseErr))?;
        let z = parts[2].parse::<u32>().or(Err(ParseErr))?;

        return Ok(Vec3{
            x, y, z
        });
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z.cmp(&other.z)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Brick {
    id: u32,
    start: Vec3,
    end: Vec3,
}

impl Brick {
    fn new(id: u32, a: Vec3, b: Vec3) -> Brick {
        return Brick{
            id: id,
            start: std::cmp::min(a, b),
            end: std::cmp::max(a, b),
        }
    }

    fn floating(&self, floor: &mut BTreeMap<(u32, u32), u32>) -> bool {
        for y in self.start.y..=self.end.y {
            for x in self.start.x..=self.end.x {
                if self.start.z - 1 <= *floor.entry((x, y)).or_insert(0) {
                    return false;
                }
            }
        }

        return true;
    }

    fn drop(&mut self, floor: &mut BTreeMap<(u32, u32), u32>) {
        let mut max_z = 0;
        for y in self.start.y..=self.end.y {
            for x in self.start.x..=self.end.x {
                let z = *floor.entry((x, y)).or_insert(0);
                max_z = std::cmp::max(max_z, z);
            }
        }

        let new_z = max_z + 1;
        let z_offs = self.start.z - new_z;
        self.start.z = new_z;
        self.end.z = self.end.z - z_offs;

        for y in self.start.y..=self.end.y {
            for x in self.start.x..=self.end.x {
                floor.insert((x, y), self.end.z);
            }
        }
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut bricks = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let (a, b) = line.split_once("~").unwrap();

        let a = Vec3::from_str(a)?;
        let b = Vec3::from_str(b)?;

        let brick  = Brick::new(bricks.len() as u32, a, b);
        bricks.push(brick);
    }

    // Sort by min Z
    bricks.sort();

    // Keep track of the "floor" as we drop bricks
    let mut floor: BTreeMap<(u32, u32), u32> = BTreeMap::new();

    // Drop them all
    for brick in bricks.iter_mut() {
        brick.drop(&mut floor);
    }

    // Re-sort in their settled positions
    bricks.sort();

    // Now look for which bricks are supported by what

    // This is a pretty ugly implementation, but the basic idea is:
    //   - To start with, all bricks are candidates for disintegration
    //   - For each brick, look at all the bricks below it
    //   - Track which bricks support it
    //   - If only one brick supports it, then that single supporter can't be
    //     disintegrated, so drop it from candidates
    let mut candidates: HashSet<u32> = HashSet::from_iter(0..bricks.len() as u32);
    for (i, b) in bricks.iter().enumerate() {
        let mut supporters = HashSet::new();
        for j in (0..i).rev() {
            let other = bricks[j];
            if other.end.z == b.start.z - 1 {
                for y in b.start.y..=b.end.y {
                    for x in b.start.x..=b.end.x {
                        if x >= other.start.x && x <= other.end.x &&
                            y >= other.start.y && y <= other.end.y {
                            supporters.insert(other.id);
                        }
                    }
                }
            }
        }
        let supporters: Vec<u32> = supporters.iter().map(|&v| v.clone()).collect();
        if supporters.len() == 1 {
            candidates.remove(&supporters[0]);
        }
    }

    println!("{}", candidates.len());

    Ok(())
}
