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

    fn find_v_mirror(&self) -> Option<usize> {
        for mirror_after in 0..self.cols - 1 {
            let mut ok = true;
            for src in 0..=mirror_after {
                if let Some(dst) = reflect(mirror_after, src, self.cols) {
                    for y in 0..self.rows {
                        let a = self.cells[y][src];
                        let b = self.cells[y][dst];

                        if a != b {
                            ok = false;
                            break;
                        }
                    }
                }
            }

            if ok {
                return Some(mirror_after + 1);
            }
        }

        return None;
    }

    fn find_h_mirror(&self) -> Option<usize> {
        for mirror_after in 0..self.rows - 1 {
            let mut ok = true;
            for src in 0..=mirror_after {
                if let Some(dst) = reflect(mirror_after, src, self.rows) {
                    for x in 0..self.cols {
                        let a = self.cells[src][x];
                        let b = self.cells[dst][x];

                        if a != b {
                            ok = false;
                            break;
                        }
                    }
                }
            }

            if ok {
                return Some(mirror_after + 1);
            }
        }

        return None;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let data = fs::read_to_string(fname)?;

    let grids = data.split("\n\n");

    let mut total = 0;

    for g in grids {
        let mut rows = Vec::new();
        for line in g.lines() {
            let cols = Vec::from_iter(line.bytes());
            rows.push(cols);
        }

        let m = Matrix::new(rows);
        if let Some(cols) = m.find_v_mirror() {
            total += cols;
        } else if let Some(rows) = m.find_h_mirror() {
            total += rows * 100;
        }
    }

    println!("{total}");

    Ok(())
}
