use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn polygon_area(verts: &Vec<(i32, i32)>) -> i64 {
    let mut area: i64 = 0;
    for i in 0..verts.len() - 1 {
        let v1 = verts[i];
        let v2 = verts[i + 1];
        let a = ((v1.1 as i64 + v2.1 as i64) * ((v1.0 as i64 - v2.0 as i64))) / 2;
        area += a;
    }

    return area;
}

fn polygon_perimeter(verts: &Vec<(i32, i32)>) -> i64 {
    let mut perimeter: i64 = 0;

    for i in 0..verts.len() - 1 {
        let v1 = verts[i];
        let v2 = verts[i + 1];
        perimeter += (v2.0 - v1.0).abs() as i64 + (v2.1 - v1.1).abs() as i64;
    }

    return perimeter;
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

    let mut p1_verts = Vec::new();
    let mut p2_verts = Vec::new();

    p1_verts.push((0, 0));
    p2_verts.push((0, 0));

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split(" ").collect();

        { // Part 1
            let dp = dig_dirs.get(parts[0]).unwrap();
            let n = parts[1].parse::<i32>().unwrap();
            let last = p1_verts.last().unwrap();

            p1_verts.push((
                last.0 + dp.0 * n,
                last.1 + dp.1 * n,
            ));
        }

        { // Part 2
            let insn = u32::from_str_radix(&parts[2][2..8], 16).unwrap();
            let dp = DIRS[(insn & 0xf) as usize];
            let n = (insn >> 4) as i32;
            let last = p2_verts.last().unwrap();

            p2_verts.push((
                last.0 + dp.0 * n,
                last.1 + dp.1 * n,
            ));
        }
    }

    let area = polygon_area(&p1_verts);
    let perimeter = polygon_perimeter(&p1_verts);
    println!("{}", area + perimeter / 2 + 1);

    let area = polygon_area(&p2_verts);
    let perimeter = polygon_perimeter(&p2_verts);
    println!("{}", area + perimeter / 2 + 1);

    Ok(())
}
