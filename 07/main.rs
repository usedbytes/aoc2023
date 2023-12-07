use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: u32,
    hand_type: HandType,
}

const CARD_ORDER: &str = "0123456789TJQKA";
const JOKERED_ORDER: &str = "J0123456789TQKA";

impl Hand {
    fn new(hand: String, bid: u32) -> Hand {
        return Hand{
            hand_type: Hand::get_type(&hand),
            hand,
            bid,
        }
    }

    fn get_type(hand: &String) -> HandType {
        let mut cards = HashMap::new();

        for letter in hand.chars() {
            *cards.entry(letter).or_insert(0) += 1
        }
        let max = cards.values().max().unwrap();

        match (cards.len(), max) {
            (5, _) => HandType::HighCard,
            (4, _) => HandType::OnePair,
            (3, 2) => HandType::TwoPair,
            (3, 3) => HandType::ThreeOfAKind,
            (2, 3) => HandType::FullHouse,
            (2, 4) => HandType::FourOfAKind,
            (1, _) => HandType::FiveOfAKind,
            _ => panic!(),
        }
    }

    fn compare(self: &Hand, other: &Hand, card_order: &str) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else {
            let bchars = other.hand.as_bytes();
            for (i, a) in self.hand.chars().enumerate() {
                let b = bchars[i];

                let aidx = card_order.find(a).unwrap();
                let bidx = card_order.find(b as char).unwrap();
                if aidx != bidx {
                    return aidx.cmp(&bidx);
                }
            }
        }

        return Ordering::Equal;
    }

    fn new_jokered(hand: String, bid: u32) -> Hand {
        return Hand{
            hand_type: Hand::get_jokered_type(&hand),
            hand,
            bid,
        }
    }

    fn get_jokered_type(hand: &String) -> HandType {
        let mut cards = HashMap::new();

        let mut jokers = 0;
        for letter in hand.chars() {
            if letter == 'J' {
                jokers += 1;
            } else {
                *cards.entry(letter).or_insert(0) += 1
            }
        }

        // This is a bit of a hack, but I'm bored now.
        if jokers == 5 {
            cards.insert('J', 5);
            jokers = 0;
        }

        // Assign the jokers to the most plentiful card face
        let max = cards.values().max().unwrap();
        for (letter, num) in cards.iter() {
            if num == max {
                cards.entry(*letter).and_modify(|v| *v += jokers);
                break;
            }
        }

        // Recalculate the max
        let max = cards.values().max().unwrap();

        match (cards.len(), max) {
            (5, _) => HandType::HighCard,
            (4, _) => HandType::OnePair,
            (3, 2) => HandType::TwoPair,
            (3, 3) => HandType::ThreeOfAKind,
            (2, 3) => HandType::FullHouse,
            (2, 4) => HandType::FourOfAKind,
            (1, _) => HandType::FiveOfAKind,
            _ => panic!(),
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut hands = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (hand, bid) =  line.split_once(" ").unwrap();

        let hand = hand.to_string();

        hands.push(
            Hand::new(
                hand.to_string(),
                bid.parse().unwrap()
            )
        );
    }

    hands.sort_by(|a, b| a.compare(b, CARD_ORDER));

    let mut score = 0;
    for (rank, hand) in hands.iter().enumerate() {
        score += (rank as u32 + 1) * hand.bid;
    }

    println!("{}", score);

    let mut jokered_hands = Vec::new();
    for hand in hands.iter() {
        jokered_hands.push(
            Hand::new_jokered(
                hand.hand.to_string(),
                hand.bid
            )
        )
    }

    jokered_hands.sort_by(|a, b| a.compare(b, JOKERED_ORDER));
    let mut score = 0;
    for (rank, hand) in jokered_hands.iter().enumerate() {
        score += (rank as u32 + 1) * hand.bid;
    }

    println!("{}", score);

    Ok(())
}
