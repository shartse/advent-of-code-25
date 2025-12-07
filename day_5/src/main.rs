use std::{
    cmp::{max, min},
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

#[derive(Debug)]
struct Inventory {
    fresh_ranges: Vec<Range>,
    ids: Vec<u64>,
}

impl Inventory {
    fn new(lines: Vec<String>) -> Self {
        let mut fresh_ranges = Vec::new();
        let mut ids = Vec::new();
        for l in lines {
            if l.len() == 0 {
                continue;
            }
            if let Some((a, b)) = l.split_once('-') {
                fresh_ranges.push(Range {
                    start: a.parse::<u64>().unwrap(),
                    end: b.parse::<u64>().unwrap(),
                });
            } else {
                ids.push(l.parse::<u64>().unwrap())
            }
        }
        fresh_ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));
        Self { fresh_ranges, ids }
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}
impl Range {
    fn overlapping(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.start <= other.end && self.end >= other.end)
    }
}

fn part_1(vals: Vec<String>) {
    let mut ans = 0;
    let inventory = Inventory::new(vals);
    for i in inventory.ids.iter() {
        for r in inventory.fresh_ranges.iter() {
            if i >= &r.start && i <= &r.end {
                ans += 1;
                break;
            }
        }
    }
    println!("Total value is: {ans}");
}

fn part_2(vals: Vec<String>) {
    let inventory = Inventory::new(vals);
    let mut cur = inventory.fresh_ranges[0].clone();
    let mut combined_ranges = Vec::new();
    for r in inventory.fresh_ranges.iter().skip(1) {
        if cur.overlapping(r) || r.overlapping(&cur) {
            cur = Range {
                start: min(cur.start, r.start),
                end: max(cur.end, r.end),
            };
        } else {
            combined_ranges.push(cur);
            cur = r.clone();
        }
    }
    combined_ranges.push(cur);
    let ans = combined_ranges.iter().map(|r| r.end - r.start + 1).sum::<u64>();
    println!("Total value is: {ans}");
}