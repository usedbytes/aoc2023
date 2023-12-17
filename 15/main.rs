use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use regex::Regex;

type Lense = (String, u32);

struct Box {
    id: u32,
    lenses: Vec<Lense>,
}

impl Box {
    fn new(id: u32) -> Box {
        return Box{
            id: id,
            lenses: Vec::new(),
        };
    }

    fn op_eq(&mut self, lense: Lense) {
        if let Some(pos) = self.lenses.iter().position(|x| x.0 == lense.0) {
            self.lenses[pos] = lense;
        } else {
            self.lenses.push(lense);
        }
    }

    fn op_minus(&mut self, label: &str) {
        if let Some(pos) = self.lenses.iter().position(|x| x.0 == label) {
            self.lenses.remove(pos);
        }
    }

    fn power(&self) -> u32 {
        let mut power = 0;
        for (i, (_, f_l)) in self.lenses.iter().enumerate() {
            power += (self.id + 1) * (i as u32 + 1) * f_l;
        }
        return power;
    }
}

fn checksum(s: &str) -> u8 {
    let mut val: u32 = 0;
    for b in s.bytes() {
        val += b as u32;
        val *= 17;
        val %= 256;
    }

    return val as u8;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total: u32 = 0;

    let insn_re = Regex::new(r"([a-z]+)([=-])([0-9])?").unwrap();

    let mut boxes = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        for insn in line.split(",") {
            let csum = checksum(insn);
            total += csum as u32;

            let caps = insn_re.captures(&insn).unwrap();

            let label = caps.get(1).unwrap().as_str();
            let box_num = checksum(&label);

            let b = boxes.entry(box_num).or_insert(Box::new(box_num.into()));

            let op = caps.get(2).unwrap().as_str();
            if op == "-" {
                b.op_minus(label);
            } else {
                let focal_length = caps.get(3)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                b.op_eq((label.to_string(), focal_length));
            }
        }
    }

    println!("{total}");

    let mut power = 0;
    for b in boxes.values() {
        power += b.power();
    }

    println!("{power}");

    Ok(())
}
