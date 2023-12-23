use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::collections::{BTreeSet, BTreeMap};
use std::fs::File;

const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn move_in_dir(
    map: &Vec<Vec<char>>,
    from: &(usize, usize),
    dir: usize,
    part2: &bool
) -> Option<(usize, usize)> {
    let dp = DIRS[dir];
    let nx = from.0 as i32 + dp.0;
    let ny = from.1 as i32 + dp.1;

    if nx < 0 || nx >= map[0].len() as i32 ||
        ny < 0 || ny >= map.len() as i32 {
        return None;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    let cell = map[ny][nx];
    match (part2, cell, dir) {
        (_, '#', _) => None,
        (false, '>', 2) => None,
        (false, 'v', 3) => None,
        (false, '<', 0) => None,
        (false, '^', 1) => None,
        (false, '.', _) => Some((nx, ny)),
        _ => Some((nx, ny)),
    }
}

type Graph = BTreeMap<(usize, usize), BTreeSet<((usize, usize), usize)>>;

fn insert_edge(graph: &mut Graph, a: &(usize, usize), b: &(usize, usize), distance: usize, bidir: &bool) {
    graph.entry(*a)
        .or_insert(BTreeSet::new())
        .insert((*b, distance));
    if *bidir {
        graph.entry(*b)
            .or_insert(BTreeSet::new())
            .insert((*a, distance));
    }
}

fn build_graph(
    map: &Vec<Vec<char>>,
    graph: &mut Graph,
    from: &(usize, usize),
    fork: &(usize, usize),
    forks: &mut BTreeSet<(usize, usize)>,
    part2: &bool) {

    let mut current = *from;
    let mut prev = *fork;

    let mut n = if *from == *fork { 0 } else { 1 };

    loop {
        let cell = map[current.1][current.0];
        let dirs = match (part2, cell) {
            (false, '>') => 0..=0,
            (false, 'v') => 1..=1,
            (false, '<') => 2..=2,
            (false, '^') => 3..=3,
            (false, _) => 0..=3,
            (true, _) => 0..=3,
        };

        let dirs: Vec<usize> = dirs.collect();

        let options: Vec<(usize, usize)> = dirs.iter()
            .filter_map(
                |d| move_in_dir(map, &current, *d, part2)
            )
            .filter(|v| *v != prev) // Can't go back - this only works if the path is 1-wide!
            .collect();

        if options.len() > 1 {
            insert_edge(graph, fork, &current, n, part2);

            if !forks.contains(&current) {
                forks.insert(current);
                for option in options {
                    build_graph(map, graph, &option, &current, forks, part2);
                }
            }

            return;
        } else if options.len() == 0 {
            insert_edge(graph, fork, &current, n, part2);

            return;
        } else {
            n += 1;
            prev = current;
            current = options[0];
        }
    }
}

fn explore_graph(
    graph: &Graph,
    from: &(usize, usize),
    goal: &(usize, usize),
    path: &mut BTreeSet<(usize, usize)>,
    distance: usize,
    results: &mut BTreeSet<usize>,
) {
    if let Some(options) = graph.get(from) {
        for (option, dist) in options {
            if path.contains(&option) {
                continue;
            }

            if option == goal {
                results.insert(distance + dist);
            } else {
                let mut path = path.clone();
                path.insert(*option);
                explore_graph(graph, option, goal, &mut path, distance + dist, results);
            }
        }
    }
}

fn graph_to_dot(graph: &Graph) {
    println!("digraph G {{");
    for (k, v) in graph {
        for (dest, dist) in v {
            println!("\"{:?}\" -> \"{:?}\"", k, dest);
        }
    }
    println!("}}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row = Vec::from_iter(line.chars());
        map.push(row);
    }

    let mut start_x = 0;
    for (i, x) in map[0].iter().enumerate() {
        if *x == '.' {
            start_x = i;
        }
    }
    let start = (start_x, 0);

    let mut end_x = 0;
    for (i, x) in map[map.len() - 1].iter().enumerate() {
        if *x == '.' {
            end_x = i;
        }
    }
    let end = (end_x, map.len() - 1);

    // Part 1
    {
        let mut graph = Graph::new();
        build_graph(&map, &mut graph, &start, &start, &mut BTreeSet::new(), &false);

        let mut results = BTreeSet::new();
        explore_graph(&graph, &start, &end, &mut BTreeSet::from([start]), 0, &mut results);
        println!("{:?}", results.iter().max().unwrap());
    }

    // Part 2
    {
        let mut graph = Graph::new();
        build_graph(&map, &mut graph, &start, &start, &mut BTreeSet::new(), &true);

        let mut results = BTreeSet::new();
        explore_graph(&graph, &start, &end, &mut BTreeSet::from([start]), 0, &mut results);
        println!("{:?}", results.iter().max().unwrap());
    }

    Ok(())
}
