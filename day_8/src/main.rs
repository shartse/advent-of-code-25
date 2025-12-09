use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    part_1(values.clone());
    part_2(values);
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point(i64, i64, i64);

impl Point {
    fn new(line: &str) -> Self {
        let v: Vec<&str> = line.split(',').collect();
        Point(
            v[0].parse::<i64>().unwrap(),
            v[1].parse::<i64>().unwrap(),
            v[2].parse::<i64>().unwrap(),
        )
    }

    fn distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pair {
    a: Point,
    b: Point,
    dist: i64,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.a.0.cmp(&self.b.0))
    }
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_pair_heap(boxes: &HashSet<Point>) -> BinaryHeap<Pair> {
    let mut pairs: HashSet<Pair> = HashSet::new();
    let mut distances: BinaryHeap<Pair> = BinaryHeap::new();
    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().enumerate() {
            if i == j {
                continue;
            }
            let dist = a.distance(b);
            let pair = Pair {
                a: if a.0 > b.0 { a.clone() } else { b.clone() },
                b: if a.0 > b.0 { b.clone() } else { a.clone() },
                dist,
            };
            pairs.insert(pair);
        }
    }
    for p in pairs.into_iter() {
        distances.push(p);
    }
    distances
}

fn step(
    boxes: &mut HashSet<Point>,
    distances: &mut BinaryHeap<Pair>,
    circuits: &mut Vec<HashSet<Point>>,
) -> Pair {
    let next = distances.pop().unwrap();
    let mut a_circuit_index = None;
    let mut b_circuit_index = None;
    for (i, c) in circuits.iter().enumerate() {
        if c.contains(&next.a) {
            a_circuit_index = Some(i)
        }
        if c.contains(&next.b) {
            b_circuit_index = Some(i)
        }
    }
    match (a_circuit_index, b_circuit_index) {
        (None, None) => {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(next.a.clone());
            new_circuit.insert(next.b.clone());
            circuits.push(new_circuit);
        }
        (None, Some(b_i)) => {
            circuits[b_i].insert(next.a.clone());
        }
        (Some(a_i), None) => {
            circuits[a_i].insert(next.b.clone());
        }
        (Some(a_i), Some(b_i)) => {
            if a_i != b_i {
                let mut new_circuit: HashSet<Point> = HashSet::new();
                let a_circuit: HashSet<Point>;
                let b_circuit: HashSet<Point>;
                if a_i > b_i {
                    a_circuit = circuits.remove(a_i);
                    b_circuit = circuits.remove(b_i);
                } else {
                    b_circuit = circuits.remove(b_i);
                    a_circuit = circuits.remove(a_i);
                }
                new_circuit.extend(a_circuit);
                new_circuit.extend(b_circuit);
                circuits.push(new_circuit);
            }
        }
    }
    boxes.remove(&next.a);
    boxes.remove(&next.b);
    return next.clone();
}

fn part_1(vals: Vec<String>) {
    let mut boxes: HashSet<Point> = vals.iter().map(|v| Point::new(v)).collect();
    let mut distances = build_pair_heap(&boxes);
    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    for _i in 0..1000 {
        step(&mut boxes, &mut distances, &mut circuits);
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let ans: usize = circuits[0..3].iter().map(|c| c.len()).product();
    println!("Part 1 answer is: {ans}");
}

fn part_2(vals: Vec<String>) {
    let mut boxes: HashSet<Point> = vals.iter().map(|v| Point::new(v)).collect();
    let mut distances: BinaryHeap<Pair> = build_pair_heap(&boxes);
    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    let last_pair: Pair;
    loop {
        let next = step(&mut boxes, &mut distances, &mut circuits);
        let circuit_count = circuits.len() + boxes.len();
        if circuit_count == 1 {
            last_pair = next.clone();
            break;
        }
    }
    let ans = last_pair.a.0 * last_pair.b.0;
    println!("Part 2 answer is: {ans}");
}