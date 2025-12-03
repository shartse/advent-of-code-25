use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("test.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    part_1(values.clone());
    part_2(values);
}

fn part_1(vals: Vec<String>) {
    let mut zero_count = 0;
    let mut number = 50;
    for v in vals.iter() {
        let (dir, val) = v.split_at(1);
        if dir == "R" {
            number = (number + val.parse::<i32>().unwrap()) % 100
        } else if dir == "L" {
            number = (number - val.parse::<i32>().unwrap()) % 100
        }
        if number < 0 {
            number = number + 100
        }
        if number == 0 {
            zero_count += 1;
        }
    }
    println!("Total value is: {zero_count}");
}

fn part_2(vals: Vec<String>) {
    let mut zero_count = 0;
    let mut start = 50;
    for v in vals.iter() {
        let (dir, val) = v.split_at(1);
        let clicks = val.parse::<i32>().unwrap();
        let rotations = clicks / 100;
        let mut dest = if dir == "R" {
            (start + clicks) % 100
        } else {
            (start - clicks) % 100
        };
        if dest < 0 {
            dest = dest + 100
        }

        if start != 0 && dest != 0 {
            if dir == "R" && dest < start {
                zero_count += 1
            }
            if dir == "L" && dest > start {
                zero_count += 1
            }
        }
        if dest == 0 {
            zero_count += 1
        }

        zero_count += rotations;
        start = dest;
    }
    println!("Total value is: {zero_count}");
}