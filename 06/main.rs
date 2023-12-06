use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn distance_with_hold(self: &Self, hold_time: u32) -> u32 {
        let speed = hold_time;
        let remainder = self.time - hold_time;
        return remainder * speed;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let time_line = lines.next().unwrap()?;
    let (_, rest) = time_line.split_once(":").unwrap();
    let times: Vec<u32> = rest.split_whitespace()
        .map(|v| v.trim().parse::<u32>().unwrap())
        .collect();

    let distance_line = lines.next().unwrap()?;
    let (_, rest) = distance_line.split_once(":").unwrap();
    let distances: Vec<u32> = rest.split_whitespace()
        .map(|v| v.trim().parse::<u32>().unwrap())
        .collect();

    let mut races = Vec::new();

    for (i, time) in times.iter().enumerate() {
        races.push(Race{
            time: *time,
            distance: distances[i],
        });
    }

    let mut total_margin = 1;
    for race in races.iter() {
        let mut num_wins = 0;
        for i in 0..=race.time {
            let distance = race.distance_with_hold(i);
            if distance > race.distance {
                num_wins += 1;
            }
        }
        total_margin *= num_wins;
    }

    println!("{total_margin}");

    Ok(())
}
