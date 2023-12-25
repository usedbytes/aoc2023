use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;

type Graph = HashMap<String, HashSet<String>>;

fn insert_edge(graph: &mut Graph, from: &str, to: &str) {
    graph.entry(from.to_string())
        .or_insert(HashSet::new())
        .insert(to.to_string());
    graph.entry(to.to_string())
        .or_insert(HashSet::new())
        .insert(from.to_string());
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    let mut graph = Graph::new();

    // Generate a .dot graph
    println!("graph G {{");
    for line in reader.lines() {
        let line = line?;

        let (a, rest) = line.split_once(": ").unwrap();

        let bs = rest.split(" ");

        for b in bs {
            println!("   {} -- {}", a, b);
            insert_edge(&mut graph, a, b);
        }

    }
    println!("}}");

    // Eyeball it, for my input, the edges to be deleted are:
    let snips = [
        ("rrz", "pzq"),
        ("jtr", "mtq"),
        ("ddj", "znv"),
    ];

    // Delete those edges
    for e in snips {
        graph.entry(e.0.to_string()).and_modify(
            |entry| { (*entry).remove(e.1); }
        );
        graph.entry(e.1.to_string()).and_modify(
            |entry| { (*entry).remove(e.0); }
        );
    }

    // Then check reachability for both sides of one of the snips
    let mut visited: HashSet<String> = HashSet::new();
    let mut to_visit: VecDeque<String> = VecDeque::new();
    to_visit.push_back(snips[0].0.to_string());
    while let Some(node) = to_visit.pop_front() {
        visited.insert(node.clone());
        for other in graph.get(&node).unwrap().iter() {
            if !visited.contains(other) {
                to_visit.push_back(other.clone());
            }
        }
    }
    let left = visited.len();

    let mut visited: HashSet<String> = HashSet::new();
    let mut to_visit: VecDeque<String> = VecDeque::new();
    to_visit.push_back(snips[0].1.to_string());
    while let Some(node) = to_visit.pop_front() {
        visited.insert(node.clone());
        for other in graph.get(&node).unwrap().iter() {
            if !visited.contains(other) {
                to_visit.push_back(other.clone());
            }
        }
    }
    let right = visited.len();

    println!("{}", left * right);

    Ok(())
}
