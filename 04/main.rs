use std::collections::HashSet;
use std::iter::FromIterator;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line?;

        let (_, rest) = line.split_once(":").unwrap();
        let (winning, card) = rest.split_once("|").unwrap();

        let winning = winning.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap());
        let winning: HashSet<u32> = HashSet::from_iter(winning);

        let card = card.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap());
        let card: HashSet<u32> = HashSet::from_iter(card);

        let wins: Vec<&u32> = card.intersection(&winning).collect();
        let n_wins = wins.len();

        let score = if n_wins > 0 { 1 << (n_wins - 1) } else { 0 }; 

        total_score += score;
    }

    println!("{total_score}");

    Ok(())
}
