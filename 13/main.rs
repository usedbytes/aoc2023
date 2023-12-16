use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Matrix {
    cells: Vec<Vec<u8>>,
    cols: usize,
    rows: usize,
}

fn reflect(mirror_after: usize, src: usize, max: usize) -> Option<usize> {
    let distance = mirror_after - src + 1;
    let dest = mirror_after + distance;
    if dest < max {
        return Some(dest);
    }
    return None;
}

impl Matrix {
    fn new(cells: Vec<Vec<u8>>) -> Matrix {
        return Matrix{
            rows: cells.len(),
            cols: cells[0].len(),
            cells: cells,
        };
    }

    fn v_mirror_diffs(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for mirror_after in 0..self.cols - 1 {
            let mut mismatches = 0;
            for src in 0..=mirror_after {
                if let Some(dst) = reflect(mirror_after, src, self.cols) {
                    for y in 0..self.rows {
                        let a = self.cells[y][src];
                        let b = self.cells[y][dst];

                        if a != b {
                            mismatches += 1;
                        }
                    }
                }
            }

            result.push(mismatches);
        }

        return result;
    }

    fn h_mirror_diffs(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for mirror_after in 0..self.rows - 1 {
            let mut mismatches = 0;
            for src in 0..=mirror_after {
                if let Some(dst) = reflect(mirror_after, src, self.rows) {
                    for x in 0..self.cols {
                        let a = self.cells[src][x];
                        let b = self.cells[dst][x];

                        if a != b {
                            mismatches += 1;
                        }
                    }
                }
            }

            result.push(mismatches);
        }

        return result;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let data = fs::read_to_string(fname)?;

    let grids = data.split("\n\n");

    let mut total = 0;
    let mut total2 = 0;

    for g in grids {
        let mut rows = Vec::new();
        for line in g.lines() {
            let cols = Vec::from_iter(line.bytes());
            rows.push(cols);
        }

        let m = Matrix::new(rows);

        let v = m.v_mirror_diffs();
        let h = m.h_mirror_diffs();

        { // Part 1 - look for zero
            let p1_cols = v.iter().position(|&val| val == 0);
            let p1_rows = h.iter().position(|&val| val == 0);

            assert!(p1_cols.is_none() || p1_rows.is_none());

            if let Some(cols) = p1_cols {
                total += cols + 1;
            } else if let Some(rows) = p1_rows {
                total += (rows + 1) * 100;
            }
        }

        { // Part 2 - look for one
            let p2_cols = v.iter().position(|&val| val == 1);
            let p2_rows = h.iter().position(|&val| val == 1);

            assert!(p2_cols.is_none() || p2_rows.is_none());

            if let Some(cols) = p2_cols {
                total2 += cols + 1;
            } else if let Some(rows) = p2_rows {
                total2 += (rows + 1) * 100;
            }
        }
    }

    println!("{total}");
    println!("{total2}");

    Ok(())
}
