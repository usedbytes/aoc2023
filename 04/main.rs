use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn evaluate_wins(
    card_wins: &Vec<Vec<u32>>,
    card_n_wins: &mut HashMap<u32, u32>,
    card: &u32) -> u32 {

    match card_n_wins.get(&card) {
        Some(n) => *n,
        None => {
            let wins = &card_wins[(card - 1) as usize];
            let mut n_wins = wins.len() as u32;

            for win in wins.iter() {
                n_wins += evaluate_wins(card_wins, card_n_wins, win);
            }

            // Add result to card_n_wins so we don't have to calculate it again
            card_n_wins.insert(*card, n_wins);

            return n_wins;
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut total_score = 0;

    // Tracks which cards are won by each card. Index is card number - 1.
    let mut card_wins: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let (card_n, rest) = line.split_once(":").unwrap();

        let (_, n) = card_n.split_once(" ").unwrap();
        let n = n.trim().parse::<u32>().unwrap();

        let (winning, card) = rest.split_once("|").unwrap();

        let winning = winning.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap());
        let winning: HashSet<u32> = HashSet::from_iter(winning);

        let card = card.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap());
        let card: HashSet<u32> = HashSet::from_iter(card);

        let n_wins = card.intersection(&winning).count();

        let wins = Range{start: n + 1, end: n + 1 + n_wins as u32};
        card_wins.push(wins.collect());

        let score = if n_wins > 0 { 1 << (n_wins - 1) } else { 0 }; 

        total_score += score;
    }

    println!("{total_score}");

    // Tracks how many cards are won by a given card number, for memoisation.
    let mut card_n_wins: HashMap<u32, u32> = HashMap::new();

    let mut total_cards = card_wins.len() as u32;
    for i in 1..=card_wins.len() {
        total_cards += evaluate_wins(&card_wins, &mut card_n_wins, &(i as u32));
    }

    println!("{total_cards}");

    Ok(())
}
