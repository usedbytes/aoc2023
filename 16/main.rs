use std::env;
use std::fs::File;
use std::collections::BTreeSet;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Ray {
    pos: (usize, usize),
    dir: (i32, i32)
}

fn valid(nrows: usize, ncols: usize, pos: (i32, i32)) -> bool {
    if pos.0 >= 0 && pos.0 < ncols as i32 &&
        pos.1 >= 0 && pos.1 < nrows as i32 {
        return true;
    }

    return false;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut cave = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = Vec::from_iter(line.chars());
        cave.push(row);
    }

    let nrows = cave.len();
    let ncols = cave[0].len();

    let mut rays = BTreeSet::new();
    rays.insert(Ray{
        pos: (0, 0),
        dir: (1, 0),
    });

    let mut energized = BTreeSet::new();
    let mut traced_rays = BTreeSet::new();

    while rays.len() > 0 {
        //println!("Rays: {:?}", rays);
        let mut ray = rays.pop_last().unwrap();
        //println!("Ray: {:?}", ray);

        'next_ray: while !traced_rays.contains(&ray) {
            energized.insert(ray.pos);
            traced_rays.insert(ray.clone());

            let cell = cave[ray.pos.1][ray.pos.0];
            match cell {
                '|' => {
                    if ray.dir.0 != 0 {
                        // Split - redirect this ray
                        ray.dir = (0, -1);

                        // And spawn a new one
                        rays.insert(Ray{
                            pos: ray.pos,
                            dir: (0, 1),
                        });
                    }
                },
                '-' => {
                    if ray.dir.1 != 0 {
                        // Split - redirect this ray
                        ray.dir = (-1, 0);

                        // And spawn a new one
                        rays.insert(Ray{
                            pos: ray.pos,
                            dir: (1, 0),
                        });
                    }
                },
                '/' => {
                    if ray.dir.0 == 1 {
                        ray.dir = (0, -1);
                    } else if ray.dir.0 == -1 {
                        ray.dir = (0, 1);
                    } else if ray.dir.1 == 1 {
                        ray.dir = (-1, 0);
                    } else if ray.dir.1 == -1 {
                        ray.dir = (1, 0);
                    } else {
                        panic!();
                    }
                },
                '\\' => {
                    if ray.dir.0 == 1 {
                        ray.dir = (0, 1);
                    } else if ray.dir.0 == -1 {
                        ray.dir = (0, -1);
                    } else if ray.dir.1 == 1 {
                        ray.dir = (1, 0);
                    } else if ray.dir.1 == -1 {
                        ray.dir = (-1, 0);
                    } else {
                        panic!();
                    }
                },
                _ => {},
            }

            let new_pos = (
                (ray.pos.0 as i32) + ray.dir.0,
                (ray.pos.1 as i32) + ray.dir.1
            );

            if valid(nrows, ncols, new_pos) {
                ray.pos = (new_pos.0 as usize, new_pos.1 as usize);
            } else {
                //println!("Ray died at {:?}", new_pos);
                break 'next_ray;
            }
        }
    }

    for y in 0..nrows {
        for x in 0..ncols {
            if energized.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("{}", energized.len());

    Ok(())
}
