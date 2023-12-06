use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u64,
}

impl Race {
    fn distance_with_hold(self: &Self, hold_time: u32) -> u64 {
        let speed: u64 = hold_time as u64;
        let remainder: u64 = (self.time - hold_time).into();
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
    let times_strs: Vec<&str> = rest.split_whitespace()
        .map(|v| v.trim())
        .collect();
    let times: Vec<u32> = times_strs
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    let distance_line = lines.next().unwrap()?;
    let (_, rest) = distance_line.split_once(":").unwrap();
    let distances_strs: Vec<&str> = rest.split_whitespace()
        .map(|v| v.trim())
        .collect();
    let distances: Vec<u64> = distances_strs
        .iter()
        .map(|v| v.parse::<u64>().unwrap())
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

    let p2_time = times_strs.join("").parse::<u32>().unwrap();
    let p2_distance = distances_strs.join("").parse::<u64>().unwrap();
    let p2_race = Race{
        time: p2_time,
        distance: p2_distance,
    };

    let mut p2_wins = 0;
    for i in 0..=p2_race.time {
        let distance = p2_race.distance_with_hold(i);
        if distance > p2_race.distance {
            p2_wins += 1;
        }
    }

    println!("{p2_wins}");

    Ok(())
}
