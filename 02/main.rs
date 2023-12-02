use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<HashMap<String, u32>>,
}

fn parse_game(line: String) -> Game {
    let mut split: Vec<&str> = line.split(':').collect();

    let game_id: Vec<&str> = split[0].split_whitespace().collect();
    let id: u32 = game_id[1].parse().unwrap();

    let mut game = Game {
        id: id,
        hands: Vec::<HashMap<String, u32>>::new(),
    };

    split = split[1].split(';').collect();
    for hand in split {
        let parts: Vec<&str> = hand.split_whitespace().collect();
        let mut cubes = HashMap::<String, u32>::new();

        for i in (0..parts.len()).step_by(2) {
            cubes.insert(
                String::from(parts[i + 1].trim_matches(|c| !char::is_alphabetic(c))),
                parts[i].parse().unwrap(),
            );
        }
        game.hands.push(cubes);
    }

    game
}

fn game_is_possible(game: &Game, bag: &HashMap<String, u32>) -> bool {
    for hand in &game.hands {
        for (color, n_cubes) in hand.into_iter() {
            match bag.get(color) {
                None => return false,
                Some(n_bag) => {
                    if n_bag < &n_cubes {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let bag = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);

    let mut sum = 0;

    for l in reader.lines() {
        let line = String::from(l?);

        let game = parse_game(line);

        if game_is_possible(&game, &bag) {
            sum += game.id;
        }
    }

    println!("{}", sum);

    Ok(())
}
