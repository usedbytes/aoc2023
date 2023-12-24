use std::env;
use std::error::Error;
use std::io::{self, BufRead};
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
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Vec3 {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        assert!(parts.len() == 3);

        let x = parts[0].trim().parse::<i64>().or(Err(ParseErr))?;
        let y = parts[1].trim().parse::<i64>().or(Err(ParseErr))?;
        let z = parts[2].trim().parse::<i64>().or(Err(ParseErr))?;

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

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        return Vec3{ x, y, z };
    }
}

fn check_intersect(s1: &(Vec3, Vec3), s2: &(Vec3, Vec3)) -> Option<(f64, f64)> {
    let x1 = s1.0.x as i128;
    let x2 = s1.1.x as i128;
    let x3 = s2.0.x as i128;
    let x4 = s2.1.x as i128;
    let y1 = s1.0.y as i128;
    let y2 = s1.1.y as i128;
    let y3 = s2.0.y as i128;
    let y4 = s2.1.y as i128;

    let t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if den == 0 {
        return None;
    }

    let t: f64 = t_num as f64 / den as f64;
    if t < 0.0 || t > 1.0 {
        return None;
    }

    let u: f64 = u_num as f64 / den as f64;
    if u < 0.0 || t > 1.0 {
        return None;
    }

    return Some((
        x1 as f64 + t * (x2 as f64 - x1 as f64),
        y1 as f64 + t * (y2 as f64 - y1 as f64),
    ));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let region_min = 200000000000000;
    let region_max = 400000000000000;

    let mut segments = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let (pos, vel) = line.split_once(" @ ").unwrap();
        let pos = Vec3::from_str(pos)?;
        let vel = Vec3::from_str(vel)?;

        let x_steps_to_min = (region_min - pos.x + (vel.x - vel.x.signum())) / vel.x; 
        let x_steps_to_max = (region_max - pos.x + (vel.x - vel.x.signum())) / vel.x; 

        let y_steps_to_min = (region_min - pos.y + (vel.y - vel.y.signum())) / vel.y; 
        let y_steps_to_max = (region_max - pos.y + (vel.y - vel.y.signum())) / vel.y; 

        let steps_entry = std::cmp::max(
            std::cmp::min(x_steps_to_min, x_steps_to_max),
            std::cmp::min(y_steps_to_min, y_steps_to_max),
        );
        let steps_exit = std::cmp::min(
            std::cmp::max(x_steps_to_min, x_steps_to_max),
            std::cmp::max(y_steps_to_min, y_steps_to_max),
        );

        let mut entry = Vec3::new(
            pos.x + vel.x * steps_entry,
            pos.y + vel.y * steps_entry,
            pos.z + vel.z * steps_entry,
        );

        let exit = Vec3::new(
            pos.x + vel.x * steps_exit,
            pos.y + vel.y * steps_exit,
            pos.z + vel.z * steps_exit,
        );

        if steps_entry < 0 && steps_exit < 0 {
            continue;
        } else if steps_entry < 0 {
            entry = pos.clone();
        }

        segments.push((entry, exit));
    }

    let mut count = 0;
    for i in 0..segments.len() {
        for j in i + 1..segments.len() {
            if let Some(intersection) = check_intersect(&segments[i], &segments[j]) {
                if intersection.0 >= region_min as f64 && intersection.0 <= region_max as f64 &&
                    intersection.1 >= region_min as f64 && intersection.1 <= region_max as f64 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
